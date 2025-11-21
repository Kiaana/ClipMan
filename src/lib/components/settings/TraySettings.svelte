<script lang="ts">
    import Card from "$lib/components/ui/Card.svelte";

    interface Settings {
        trayTextLength: number;
        maxPinnedInTray: number;
        maxRecentInTray: number;
        [key: string]: any;
    }

    let { settings = $bindable() } = $props<{
        settings: Settings;
    }>();
</script>

<div class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
    <div>
        <h2 class="text-lg font-semibold mb-1">托盘菜单</h2>
        <p class="text-sm text-muted-foreground">
            自定义系统托盘菜单的显示内容
        </p>
    </div>

    <Card class="p-6 space-y-6">
        <div class="space-y-4">
            <div class="flex justify-between">
                <label for="tray-text-length" class="text-sm font-medium"
                    >文本长度限制</label
                >
                <span class="text-sm font-bold text-primary"
                    >{settings.trayTextLength}</span
                >
            </div>
            <input
                id="tray-text-length"
                type="range"
                min="10"
                max="100"
                step="5"
                bind:value={settings.trayTextLength}
                class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
            />
            <p class="text-xs text-muted-foreground">
                托盘菜单中显示的文本最大长度
            </p>
        </div>

        <div class="space-y-4">
            <div class="flex justify-between">
                <label for="max-pinned" class="text-sm font-medium"
                    >最多置顶项</label
                >
                <span class="text-sm font-bold text-primary"
                    >{settings.maxPinnedInTray}</span
                >
            </div>
            <input
                id="max-pinned"
                type="range"
                min="0"
                max="10"
                step="1"
                bind:value={settings.maxPinnedInTray}
                class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
            />
            <p class="text-xs text-muted-foreground">
                托盘菜单中显示的最大置顶条目数
            </p>
        </div>

        <div class="space-y-4">
            <div class="flex justify-between">
                <label for="max-recent" class="text-sm font-medium"
                    >最多最近项</label
                >
                <span class="text-sm font-bold text-primary"
                    >{settings.maxRecentInTray}</span
                >
            </div>
            <input
                id="max-recent"
                type="range"
                min="0"
                max="30"
                step="1"
                bind:value={settings.maxRecentInTray}
                class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
            />
            <p class="text-xs text-muted-foreground">
                托盘菜单中显示的最大最近条目数
            </p>
        </div>
    </Card>
</div>
