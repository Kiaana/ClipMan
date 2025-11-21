<script lang="ts">
    import Card from "$lib/components/ui/Card.svelte";
    import Button from "$lib/components/ui/Button.svelte";
    import { Trash2 } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface Settings {
        maxHistoryItems: number;
        autoCleanup: boolean;
        storeOriginalImage: boolean;
        [key: string]: any;
    }

    let { settings = $bindable() } = $props<{
        settings: Settings;
    }>();

    let clearing = $state(false);

    async function clearAllHistory() {
        if (
            !confirm(
                "确定要清空所有剪贴板历史吗？\n\n这将删除所有记录，包括置顶项，此操作不可撤销！",
            )
        ) {
            return;
        }

        try {
            clearing = true;
            await invoke("clear_all_history");
            // Success feedback will be shown by parent component
        } catch (err) {
            console.error("Failed to clear all history:", err);
            alert("清空历史失败: " + String(err));
        } finally {
            clearing = false;
        }
    }
</script>

<div class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
    <div>
        <h2 class="text-lg font-semibold mb-1">剪贴板</h2>
        <p class="text-sm text-muted-foreground">
            管理剪贴板历史记录的存储和清理
        </p>
    </div>

    <Card class="p-6 space-y-6">
        <div class="space-y-4">
            <div class="flex justify-between">
                <div class="space-y-0.5">
                    <label for="max-items" class="text-sm font-medium"
                        >最大历史条目数</label
                    >
                    <p class="text-xs text-muted-foreground">
                        保留的历史记录数量上限
                    </p>
                </div>
                <span class="text-sm font-bold text-primary"
                    >{settings.maxHistoryItems}</span
                >
            </div>
            <input
                id="max-items"
                type="range"
                min="50"
                max="500"
                step="50"
                bind:value={settings.maxHistoryItems}
                class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
            />
        </div>

        <div class="flex items-center justify-between">
            <div class="space-y-0.5">
                <label
                    for="auto-cleanup"
                    class="text-sm font-medium cursor-pointer"
                >
                    自动清理
                </label>
                <p class="text-xs text-muted-foreground">
                    定期清理超出限制的历史记录
                </p>
            </div>
            <input
                id="auto-cleanup"
                type="checkbox"
                bind:checked={settings.autoCleanup}
                class="w-11 h-6 appearance-none rounded-full relative cursor-pointer transition-colors
                       before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                       checked:before:translate-x-5"
                style:background-color={settings.autoCleanup
                    ? "var(--primary)"
                    : "var(--muted)"}
            />
        </div>

        <div class="flex items-center justify-between">
            <div class="space-y-0.5">
                <label
                    for="store-original-image"
                    class="text-sm font-medium cursor-pointer"
                >
                    保存高质量图片
                </label>
                <p class="text-xs text-muted-foreground">
                    开启后保存原图，占用更多空间
                </p>
            </div>
            <input
                id="store-original-image"
                type="checkbox"
                bind:checked={settings.storeOriginalImage}
                class="w-11 h-6 appearance-none rounded-full relative cursor-pointer transition-colors
                       before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                       checked:before:translate-x-5"
                style:background-color={settings.storeOriginalImage
                    ? "var(--primary)"
                    : "var(--muted)"}
            />
        </div>

        <div class="pt-4 border-t border-border">
            <div class="flex items-center justify-between">
                <div class="space-y-0.5">
                    <span class="text-sm font-medium">清空所有历史</span>
                    <p class="text-xs text-muted-foreground">
                        删除所有剪贴板记录（包括置顶项）
                    </p>
                </div>
                <Button
                    type="button"
                    variant="destructive"
                    onclick={clearAllHistory}
                    disabled={clearing}
                    class="gap-2"
                >
                    <Trash2 class="h-4 w-4" />
                    {clearing ? "清空中..." : "清空所有"}
                </Button>
            </div>
        </div>
    </Card>
</div>
