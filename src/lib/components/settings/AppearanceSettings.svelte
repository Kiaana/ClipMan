<script lang="ts">
    import Card from "$lib/components/ui/Card.svelte";
    import { themeStore } from "$lib/stores/theme.svelte";
    import { Monitor, Moon, Sun, Heart } from "lucide-svelte";

    const themes = [
        { value: "light", label: "浅色", icon: Sun },
        { value: "dark", label: "深色", icon: Moon },
        { value: "light-pink", label: "淡粉色", icon: Heart },
        { value: "system", label: "跟随系统", icon: Monitor },
    ] as const;
</script>

<div class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
    <div>
        <h2 class="text-lg font-semibold mb-1">外观</h2>
        <p class="text-sm text-muted-foreground">自定义应用的主题和外观</p>
    </div>

    <Card class="p-6 space-y-6">
        <div class="space-y-3">
            <label class="text-sm font-medium">主题模式</label>
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                {#each themes as theme}
                    <button
                        class="flex flex-col items-center gap-3 p-4 rounded-lg border-2 transition-all hover:bg-muted/50
                        {themeStore.current === theme.value
                            ? 'border-primary bg-primary/5'
                            : 'border-transparent bg-muted/20'}"
                        onclick={() => themeStore.setTheme(theme.value)}
                    >
                        <div
                            class="p-2 rounded-full {themeStore.current ===
                            theme.value
                                ? 'bg-primary text-primary-foreground'
                                : 'bg-background text-muted-foreground'}"
                        >
                            <theme.icon class="h-5 w-5" />
                        </div>
                        <span class="text-sm font-medium">{theme.label}</span>
                    </button>
                {/each}
            </div>
        </div>
    </Card>
</div>
