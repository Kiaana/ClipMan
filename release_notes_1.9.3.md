# ClipMan v1.9.3 Release Notes

## 🚀 Improvements

### Code Architecture
- **Modular Refactoring**: Split `main.rs` (1232→317 lines) into dedicated modules:
  - `commands.rs` - All Tauri command handlers
  - `tray.rs` - System tray menu logic
- **Unified Copy Logic**: Merged duplicate clipboard copy functions into `copy_clip_to_clipboard_internal()`
- **Efficient Database Queries**: Added `get_by_id()` method to avoid full table scans

### User Experience
- **Copy Moves to Top**: When copying from history or tray menu, the item now moves to the top of the list (same behavior as re-copying content)

## 🔧 Technical Changes
- Added `update_timestamp()` method for reusable timestamp updates
- Created `src/lib/types.ts` for shared TypeScript type definitions
- Cleaned up temporary test files

---

# ClipMan v1.9.3 更新日志

## 🚀 改进优化

### 代码架构
- **模块化重构**：将 `main.rs`（1232→317行）拆分为独立模块：
  - `commands.rs` - 所有 Tauri 命令处理
  - `tray.rs` - 系统托盘菜单逻辑
- **统一复制逻辑**：合并重复的剪切板复制函数为 `copy_clip_to_clipboard_internal()`
- **高效数据库查询**：新增 `get_by_id()` 方法，避免全表扫描

### 用户体验
- **复制自动置顶**：从历史记录或托盘菜单复制内容时，该项会自动移到列表顶部（与重复复制内容的行为一致）

## 🔧 技术变更
- 新增 `update_timestamp()` 方法用于复用时间戳更新逻辑
- 创建 `src/lib/types.ts` 统一管理前端 TypeScript 类型定义
- 清理临时测试文件

---

**Full Changelog**: https://github.com/RustyPiano/ClipMan/compare/v1.9.2...v1.9.3
