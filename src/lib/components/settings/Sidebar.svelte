<script lang="ts">
    import {
        Settings as SettingsIcon,
        ClipboardList,
        Menu,
        Database,
        Info,
        Palette,
    } from "lucide-svelte";

    type Tab =
        | "general"
        | "clipboard"
        | "tray"
        | "storage"
        | "about"
        | "appearance"; // Updated Tab type

    let { activeTab = $bindable() } = $props<{
        activeTab: Tab;
    }>();

    const tabs = [
        // Changed type definition and added 'as const'
        { id: "general", label: "常规", icon: SettingsIcon }, // Changed Settings to SettingsIcon to match import
        { id: "appearance", label: "外观", icon: Palette }, // Added Appearance tab
        { id: "clipboard", label: "剪贴板", icon: ClipboardList },
        { id: "tray", label: "托盘菜单", icon: Menu },
        { id: "storage", label: "存储", icon: Database }, // Changed label from "数据存储" to "存储"
        { id: "about", label: "关于", icon: Info },
    ] as const;
</script>

<aside class="w-64 border-r border-border bg-muted/30 p-4 flex flex-col">
    <nav class="space-y-1">
        {#each tabs as tab}
            <button
                class="w-full flex items-center gap-3 px-3 py-2 text-sm font-medium rounded-md transition-colors
                {activeTab === tab.id
                    ? 'bg-primary text-primary-foreground shadow-sm'
                    : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
                onclick={() => (activeTab = tab.id)}
            >
                <tab.icon class="h-4 w-4" />
                {tab.label}
            </button>
        {/each}
    </nav>
</aside>
