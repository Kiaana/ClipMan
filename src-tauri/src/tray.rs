// Tray menu management module
use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder, IconMenuItemBuilder};
use lru::LruCache;
use std::sync::Mutex;
use std::num::NonZeroUsize;

use crate::storage::{ClipItem, ContentType};
use crate::AppState;

// Tray configuration constants
pub const TRAY_ICON_SIZE: u32 = 32;
const ICON_CACHE_SIZE: usize = 50;

/// Icon cache for tray menu images
pub struct TrayIconCache {
    cache: Mutex<LruCache<String, tauri::image::Image<'static>>>,
}

impl TrayIconCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(ICON_CACHE_SIZE).unwrap())),
        }
    }

    pub fn get_or_create(&self, id: &str, content: &[u8]) -> Option<tauri::image::Image<'static>> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(icon) = cache.get(id) {
                log::debug!("🎯 Icon cache hit for {}", id);
                return Some(icon.clone());
            }
        }

        // Cache miss - decode and resize image
        log::debug!("📸 Icon cache miss for {}, decoding...", id);
        match image::load_from_memory(content) {
            Ok(img) => {
                
                // Resize so shortest side is TRAY_ICON_SIZE, preserving aspect ratio
                let (orig_width, orig_height) = (img.width(), img.height());
                let min_side = orig_width.min(orig_height);
                let scale = TRAY_ICON_SIZE as f32 / min_side as f32;
                
                let new_width = (orig_width as f32 * scale) as u32;
                let new_height = (orig_height as f32 * scale) as u32;
                
                let resized = img.resize_exact(
                    new_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                );
                let width = resized.width();
                let height = resized.height();
                let rgba = resized.to_rgba8().into_raw();
                
                // Create owned image for caching
                let icon = tauri::image::Image::new_owned(rgba, width, height);
                
                // Cache it
                {
                    let mut cache = self.cache.lock().unwrap();
                    cache.put(id.to_string(), icon.clone());
                }
                
                Some(icon)
            }
            Err(e) => {
                log::warn!("Failed to decode image for clip {}: {}", id, e);
                None
            }
        }
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        log::info!("Icon cache cleared");
    }
}

/// Truncate content for menu display (handles Unicode safely)
pub fn truncate_content(content: &[u8], content_type: &ContentType, max_len: usize) -> String {
    match content_type {
        ContentType::Text | ContentType::Html | ContentType::Rtf => {
            let text = String::from_utf8_lossy(content);
            // Replace newlines and carriage returns, then collapse whitespace
            let text: String = text.chars()
                .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            // Smart truncation: show start...end for long text
            let char_count = text.chars().count();
            if char_count > max_len {
                let start_len = max_len * 2 / 3;
                let end_len = max_len - start_len - 3;
                
                let start: String = text.chars().take(start_len).collect();
                let end: String = text.chars().skip(char_count - end_len).collect();
                format!("{}...{}", start, end)
            } else {
                text
            }
        }
        ContentType::Image => "图片".to_string(),
        ContentType::File => {
            match std::str::from_utf8(content) {
                Ok(path_str) => {
                    let path = std::path::Path::new(path_str);
                    if let Some(file_name) = path.file_name() {
                        format!("文件: {}", file_name.to_string_lossy())
                    } else {
                        "文件".to_string()
                    }
                }
                Err(e) => {
                    log::warn!("Invalid UTF-8 in file path content: {}", e);
                    "文件".to_string()
                }
            }
        },
    }
}

/// Helper function to add a clip menu item
fn add_clip_menu_item(
    app: &AppHandle,
    item: &ClipItem,
    icon_cache: &TrayIconCache,
    max_len: usize,
) -> Result<Box<dyn tauri::menu::IsMenuItem<tauri::Wry>>, tauri::Error> {
    let preview = truncate_content(&item.content, &item.content_type, max_len);
    
    if matches!(item.content_type, ContentType::Image) {
        if let Some(icon) = icon_cache.get_or_create(&item.id, &item.content) {
            let menu_item = IconMenuItemBuilder::with_id(
                format!("clip:{}", item.id),
                preview
            )
            .icon(icon)
            .build(app)?;
            Ok(Box::new(menu_item))
        } else {
            let menu_item = MenuItemBuilder::with_id(
                format!("clip:{}", item.id),
                preview
            ).build(app)?;
            Ok(Box::new(menu_item))
        }
    } else {
        let menu_item = MenuItemBuilder::with_id(
            format!("clip:{}", item.id),
            preview
        ).build(app)?;
        Ok(Box::new(menu_item))
    }
}

/// Build dynamic tray menu
pub fn build_tray_menu(app: &AppHandle) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    let state = app.state::<AppState>();
    
    // Get settings for tray menu limits
    let settings = state.settings.get();
    let max_pinned_in_tray = settings.max_pinned_in_tray;
    let max_recent_in_tray = settings.max_recent_in_tray;
    let max_len = settings.tray_text_length;
    
    // Calculate query limit
    let query_limit = (max_recent_in_tray + max_pinned_in_tray).max(30);
    
    // Quick lock acquisition - get data and release immediately
    let (pinned_items, recent_items) = {
        let storage = crate::safe_lock(&state.storage);
        (
            storage.get_pinned().unwrap_or_default(),
            storage.get_recent(query_limit).unwrap_or_default(),
        )
    };
    
    let mut menu_builder = MenuBuilder::new(app);

    // Add pinned items
    let pinned_count = pinned_items.len().min(max_pinned_in_tray);
    if pinned_count > 0 {
        let pinned_header = MenuItemBuilder::with_id("pinned_header", "置顶项").enabled(false).build(app)?;
        menu_builder = menu_builder.item(&pinned_header);

        for item in pinned_items.iter().take(max_pinned_in_tray) {
            let menu_item = add_clip_menu_item(app, item, &state.icon_cache, max_len)?;
            menu_builder = menu_builder.item(&*menu_item);
        }

        menu_builder = menu_builder.separator();
    }

    // Add recent items (excluding pinned)
    let recent_unpinned: Vec<_> = recent_items.iter()
        .filter(|item| !item.is_pinned)
        .take(max_recent_in_tray)
        .collect();

    if !recent_unpinned.is_empty() {
        let recent_header = MenuItemBuilder::with_id("recent_header", "最近复制").enabled(false).build(app)?;
        menu_builder = menu_builder.item(&recent_header);

        for item in recent_unpinned {
            let menu_item = add_clip_menu_item(app, item, &state.icon_cache, max_len)?;
            menu_builder = menu_builder.item(&*menu_item);
        }
    }

    // Bottom actions
    menu_builder = menu_builder
        .separator()
        .item(&MenuItemBuilder::with_id("clear_non_pinned", "清除").build(app)?)
        .item(&MenuItemBuilder::with_id("settings", "设置").build(app)?)
        .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?);

    menu_builder.build()
}

/// Update tray menu
pub fn update_tray_menu(app: &AppHandle) {
    if let Ok(new_menu) = build_tray_menu(app) {
        if let Some(tray) = app.tray_by_id("main") {
            if let Err(e) = tray.set_menu(Some(new_menu)) {
                log::error!("Failed to update tray menu: {}", e);
            } else {
                log::debug!("Tray menu updated successfully");
            }
        }
    }
}
