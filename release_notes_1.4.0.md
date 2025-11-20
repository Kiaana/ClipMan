# ClipMan v1.4.0 Release Notes

## 🚀 Performance Improvements
- **Virtual List Implementation**: Introduced a virtualized list for clipboard history. This significantly improves rendering performance and memory usage, especially when scrolling through hundreds of clipboard items. The UI should now feel much smoother and more responsive.
- **Optimized Filtering**: Removed redundant client-side filtering logic to reduce CPU usage during search.

## 🐛 Bug Fixes
- **Notification Permissions**: Fixed an issue where notifications would fail due to missing permission configurations in the security capabilities.

## 🛠️ Under the Hood
- Updated dependencies.
- Refactored frontend store logic for better maintainability.
