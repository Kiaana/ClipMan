<script lang="ts">
    import Card from "$lib/components/ui/Card.svelte";
    import Input from "$lib/components/ui/Input.svelte";
    import Button from "$lib/components/ui/Button.svelte";

    interface Settings {
        globalShortcut: string;
        enableAutostart: boolean;
        [key: string]: any;
    }

    let { settings = $bindable() } = $props<{
        settings: Settings;
    }>();

    const shortcutPresets = [
        { label: "Cmd+Shift+V", value: "CommandOrControl+Shift+V" },
        { label: "Cmd+Shift+C", value: "CommandOrControl+Shift+C" },
        { label: "Alt+V", value: "Alt+V" },
    ];
</script>

<div class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
    <div>
        <h2 class="text-lg font-semibold mb-1">常规</h2>
        <p class="text-sm text-muted-foreground">应用的基本行为设置</p>
    </div>

    <Card class="p-6 space-y-6">
        <div class="flex items-center justify-between">
            <div class="space-y-0.5">
                <label
                    for="enable-autostart"
                    class="text-sm font-medium cursor-pointer"
                >
                    开机自启动
                </label>
                <p class="text-xs text-muted-foreground">
                    系统启动时自动运行 ClipMan
                </p>
            </div>
            <input
                id="enable-autostart"
                type="checkbox"
                bind:checked={settings.enableAutostart}
                class="w-11 h-6 appearance-none rounded-full relative cursor-pointer transition-colors
                       before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                       checked:before:translate-x-5"
                style:background-color={settings.enableAutostart
                    ? "var(--primary)"
                    : "var(--muted)"}
            />
        </div>

        <div class="space-y-3">
            <div class="space-y-1">
                <label for="shortcut-input" class="text-sm font-medium"
                    >全局热键</label
                >
                <p class="text-xs text-muted-foreground">
                    设置打开 ClipMan 窗口的快捷键
                </p>
            </div>
            <Input
                id="shortcut-input"
                type="text"
                bind:value={settings.globalShortcut}
                placeholder="例如: CommandOrControl+Shift+V"
            />
            <div class="flex flex-wrap gap-2 pt-1">
                {#each shortcutPresets as preset}
                    <Button
                        type="button"
                        variant={settings.globalShortcut === preset.value
                            ? "default"
                            : "outline"}
                        size="xs"
                        onclick={() => (settings.globalShortcut = preset.value)}
                    >
                        {preset.label}
                    </Button>
                {/each}
            </div>
        </div>
    </Card>
</div>
