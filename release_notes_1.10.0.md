# ClipMan v1.10.0 Release Notes

## ✨ New Features

### Internationalization (i18n)
- **Multi-language Support**: Added English and Chinese (Simplified) support
- **Auto-detect System Language**: Automatically uses system language preference
- **Language Switcher**: New language selector in Settings → Appearance
- **Persistent Preference**: Language choice saved to localStorage

## 🚀 Improvements

### Code Architecture
- **Modular Refactoring**: Split `main.rs` (1232→317 lines) into dedicated modules
- **Unified Copy Logic**: Merged duplicate clipboard copy functions
- **Efficient Database Queries**: Added `get_by_id()` method

### User Experience
- **Copy Moves to Top**: Copied items now move to the top of the list

---

# ClipMan v1.10.0 更新日志

## ✨ 新功能

### 国际化 (i18n)
- **多语言支持**：新增英文和简体中文支持
- **自动检测系统语言**：自动使用系统语言偏好
- **语言切换器**：设置 → 外观中新增语言选择器
- **持久化偏好**：语言选择保存到 localStorage

## 🚀 改进优化

### 代码架构
- **模块化重构**：将 `main.rs`（1232→317行）拆分为独立模块
- **统一复制逻辑**：合并重复的剪切板复制函数
- **高效数据库查询**：新增 `get_by_id()` 方法

### 用户体验
- **复制自动置顶**：复制的内容会自动移到列表顶部

---

**Full Changelog**: https://github.com/RustyPiano/ClipMan/compare/v1.9.3...v1.10.0
