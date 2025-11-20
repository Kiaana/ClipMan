# ClipMan v1.7.0 Release Notes

## ✨ 新功能
- **可配置托盘菜单项数量**: 新增设置选项，允许用户自定义托盘菜单中显示的项目数量
  - 置顶项：可设置 3-10 条
  - 最近项：可设置 10-50 条
  - 用户可根据自己的屏幕大小和使用习惯灵活调整

## 🛠️ 改进
- **自动 Release Notes**: GitHub Release 现在会自动包含详细的更新说明
  - Tauri 更新器会显示完整的版本变更内容
  - 用户可以在更新前了解新版本的改进

## 🐛 修复
- 移除了未使用的代码以消除编译警告

## 📊 技术细节
- 将硬编码的托盘菜单常量移至用户设置
- 添加了新的设置字段：`maxPinnedInTray` 和 `maxRecentInTray`
- 更新了 `build_tray_menu` 函数以使用动态设置

---

**Full Changelog**: https://github.com/Kiaana/ClipMan/compare/v1.6.0...v1.7.0
