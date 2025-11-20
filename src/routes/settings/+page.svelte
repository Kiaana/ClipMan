<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { router } from "$lib/stores/router.svelte";
    import Button from "$lib/components/ui/Button.svelte";
    import Input from "$lib/components/ui/Input.svelte";
    import Card from "$lib/components/ui/Card.svelte";
    import {
        ChevronLeft,
        Keyboard,
        History,
        Info,
        Loader2,
        RefreshCw,
        Download,
        Save,
        RotateCcw,
        Settings as SettingsIcon,
        Database,
        Menu,
        ClipboardList,
    } from "lucide-svelte";

    interface Settings {
        globalShortcut: string;
        maxHistoryItems: number;
        autoCleanup: boolean;
        trayTextLength: number;
        storeOriginalImage: boolean;
        maxPinnedInTray: number;
        maxRecentInTray: number;
        customDataPath: string | null;
        enableAutostart: boolean;
    }

    interface UpdateInfo {
        available: boolean;
        current_version: string;
        latest_version?: string;
        body?: string;
        date?: string;
    }

    let settings = $state<Settings>({
        globalShortcut: "CommandOrControl+Shift+V",
        maxHistoryItems: 100,
        autoCleanup: true,
        trayTextLength: 50,
        storeOriginalImage: false,
        maxPinnedInTray: 5,
        maxRecentInTray: 20,
        customDataPath: null,
        enableAutostart: false,
    });

    let loading = $state(true);
    let saving = $state(false);
    let message = $state("");

    // 更新相关状态
    let updateInfo = $state<UpdateInfo | null>(null);
    let checkingUpdate = $state(false);
    let installingUpdate = $state(false);
    let updateMessage = $state("");

    // 数据位置相关状态
    let currentDataPath = $state("");
    let changingDataPath = $state(false);
    let showMigrationDialog = $state(false);
    let newDataPath = $state("");
    let deleteOldData = $state(true);

    // 侧边栏导航状态
    type Tab = "general" | "clipboard" | "tray" | "storage" | "about";
    let activeTab = $state<Tab>("general");

    const tabs: { id: Tab; label: string; icon: any }[] = [
        { id: "general", label: "常规", icon: SettingsIcon },
        { id: "clipboard", label: "剪贴板", icon: ClipboardList },
        { id: "tray", label: "托盘菜单", icon: Menu },
        { id: "storage", label: "数据存储", icon: Database },
        { id: "about", label: "关于", icon: Info },
    ];

    onMount(async () => {
        await loadSettings();
        await loadDataPath();
    });

    async function loadSettings() {
        try {
            loading = true;
            settings = await invoke<Settings>("get_settings");
        } catch (err) {
            console.error("Failed to load settings:", err);
            const errorMsg = err instanceof Error ? err.message : String(err);
            message = "加载设置失败: " + errorMsg;
        } finally {
            loading = false;
        }
    }

    async function saveSettings() {
        try {
            saving = true;
            message = "";
            await invoke("update_settings", { settings });
            message = "设置已保存！";
            setTimeout(() => (message = ""), 3000);
        } catch (err) {
            console.error("Failed to save settings:", err);
            const errorMsg = err instanceof Error ? err.message : String(err);
            message = "保存失败: " + errorMsg;
        } finally {
            saving = false;
        }
    }

    // 常用热键预设
    const shortcutPresets = [
        {
            label: "Ctrl/Cmd + Shift + V (默认)",
            value: "CommandOrControl+Shift+V",
        },
        { label: "Ctrl/Cmd + Alt + V", value: "CommandOrControl+Alt+V" },
        { label: "Ctrl/Cmd + Shift + C", value: "CommandOrControl+Shift+C" },
        { label: "Alt + V", value: "Alt+V" },
        { label: "Ctrl/Cmd + `", value: "CommandOrControl+`" },
    ];

    // 检查更新
    async function checkForUpdates() {
        try {
            checkingUpdate = true;
            updateMessage = "";
            updateInfo = await invoke<UpdateInfo>("check_for_updates");

            if (updateInfo.available) {
                updateMessage = `发现新版本 ${updateInfo.latest_version}！`;
            } else {
                updateMessage = "当前已是最新版本";
            }
        } catch (err) {
            console.error("Failed to check for updates:", err);
            const errStr = String(err);
            updateMessage = "检查更新失败: " + errStr;
            if (errStr.includes("Not Found") || errStr.includes("404")) {
                updateMessage =
                    "检查更新失败: 未找到更新信息 (可能是尚未发布新版本)";
            }
        } finally {
            checkingUpdate = false;
        }
    }

    // 安装更新
    async function installUpdate() {
        if (!updateInfo?.available) return;

        try {
            installingUpdate = true;
            updateMessage = "正在下载并安装更新...";
            await invoke("install_update");
            updateMessage = "更新安装成功！应用将重启。";
        } catch (err) {
            console.error("Failed to install update:", err);
            const errorMsg = err instanceof Error ? err.message : String(err);
            updateMessage = "安装更新失败: " + errorMsg;
            installingUpdate = false;
        }
    }

    // 加载当前数据路径
    async function loadDataPath() {
        try {
            currentDataPath = await invoke<string>("get_current_data_path");
        } catch (err) {
            console.error("Failed to load data path:", err);
        }
    }

    // 选择新的数据存储位置
    async function changeDataLocation() {
        try {
            changingDataPath = true;
            const selectedPath = await invoke<string | null>(
                "choose_data_folder",
            );

            if (selectedPath) {
                newDataPath = selectedPath;
                showMigrationDialog = true;
            }
        } catch (err) {
            console.error("Failed to choose folder:", err);
            message = "选择文件夹失败: " + String(err);
        } finally {
            changingDataPath = false;
        }
    }

    // 确认迁移数据
    async function confirmMigration() {
        try {
            changingDataPath = true;
            message = "正在迁移数据...";

            await invoke("migrate_data_location", {
                newPath: newDataPath,
                deleteOld: deleteOldData,
            });

            message = "数据迁移成功！";
            showMigrationDialog = false;
            await loadDataPath();
            await loadSettings();

            setTimeout(() => (message = ""), 3000);
        } catch (err) {
            console.error("Failed to migrate data:", err);
            message = "数据迁移失败: " + String(err);
        } finally {
            changingDataPath = false;
        }
    }

    // 打开数据文件夹
    async function openDataFolder() {
        if (currentDataPath) {
            try {
                await invoke("open_folder", { path: currentDataPath });
            } catch (err) {
                console.error("Failed to open folder:", err);
                message = "打开文件夹失败: " + String(err);
            }
        }
    }
</script>

<div
    class="h-screen bg-background text-foreground flex flex-col overflow-hidden"
>
    <!-- 顶部标题栏 -->
    <header
        class="flex items-center gap-4 p-4 border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 z-10"
    >
        <Button variant="ghost" size="sm" onclick={() => router.goHome()}>
            <ChevronLeft class="h-4 w-4 mr-1" /> 返回
        </Button>
        <div>
            <h1 class="text-xl font-bold">设置</h1>
        </div>
        <div class="ml-auto flex gap-2">
            <Button
                variant="secondary"
                size="sm"
                onclick={loadSettings}
                disabled={loading || saving}
            >
                <RotateCcw class="h-4 w-4 mr-2" /> 重置
            </Button>
            <Button
                size="sm"
                onclick={saveSettings}
                disabled={loading || saving}
            >
                {#if saving}
                    <Loader2 class="h-4 w-4 animate-spin mr-2" /> 保存中...
                {:else}
                    <Save class="h-4 w-4 mr-2" /> 保存设置
                {/if}
            </Button>
        </div>
    </header>

    <!-- 主内容区 -->
    <div class="flex flex-1 overflow-hidden">
        <!-- 侧边栏 -->
        <aside class="w-48 border-r border-border bg-muted/30 p-2 space-y-1">
            {#each tabs as tab}
                <button
                    class="w-full flex items-center gap-2 px-3 py-2 text-sm font-medium rounded-md transition-colors
                           {activeTab === tab.id
                        ? 'bg-primary text-primary-foreground'
                        : 'hover:bg-muted text-muted-foreground hover:text-foreground'}"
                    onclick={() => (activeTab = tab.id)}
                >
                    <tab.icon class="h-4 w-4" />
                    {tab.label}
                </button>
            {/each}
        </aside>

        <!-- 设置内容 -->
        <main class="flex-1 overflow-y-auto p-6">
            {#if loading}
                <div class="flex justify-center py-12 text-muted-foreground">
                    <Loader2 class="h-6 w-6 animate-spin mr-2" /> 加载中...
                </div>
            {:else}
                <div class="max-w-2xl mx-auto space-y-6">
                    {#if activeTab === "general"}
                        <!-- 常规设置 -->
                        <div
                            class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300"
                        >
                            <div>
                                <h2 class="text-lg font-semibold mb-1">常规</h2>
                                <p class="text-sm text-muted-foreground">
                                    应用的基本行为设置
                                </p>
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
                                        <p
                                            class="text-xs text-muted-foreground"
                                        >
                                            系统启动时自动运行 ClipMan
                                        </p>
                                    </div>
                                    <input
                                        id="enable-autostart"
                                        type="checkbox"
                                        bind:checked={settings.enableAutostart}
                                        class="w-11 h-6 appearance-none bg-gray-200 dark:bg-gray-700 rounded-full relative cursor-pointer transition-colors
                                               checked:bg-primary
                                               before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                                               checked:before:translate-x-5"
                                    />
                                </div>

                                <div class="space-y-3">
                                    <div class="space-y-1">
                                        <label
                                            for="shortcut-input"
                                            class="text-sm font-medium"
                                            >全局热键</label
                                        >
                                        <p
                                            class="text-xs text-muted-foreground"
                                        >
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
                                                variant={settings.globalShortcut ===
                                                preset.value
                                                    ? "default"
                                                    : "outline"}
                                                size="xs"
                                                onclick={() =>
                                                    (settings.globalShortcut =
                                                        preset.value)}
                                            >
                                                {preset.label}
                                            </Button>
                                        {/each}
                                    </div>
                                </div>
                            </Card>
                        </div>
                    {:else if activeTab === "clipboard"}
                        <!-- 剪贴板设置 -->
                        <div
                            class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300"
                        >
                            <div>
                                <h2 class="text-lg font-semibold mb-1">
                                    剪贴板
                                </h2>
                                <p class="text-sm text-muted-foreground">
                                    管理剪贴板历史记录的存储和清理
                                </p>
                            </div>

                            <Card class="p-6 space-y-6">
                                <div class="space-y-4">
                                    <div class="flex justify-between">
                                        <div class="space-y-0.5">
                                            <label
                                                for="max-items"
                                                class="text-sm font-medium"
                                                >最大历史条目数</label
                                            >
                                            <p
                                                class="text-xs text-muted-foreground"
                                            >
                                                保留的历史记录数量上限
                                            </p>
                                        </div>
                                        <span
                                            class="text-sm font-bold text-primary"
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
                                        <p
                                            class="text-xs text-muted-foreground"
                                        >
                                            定期清理超出限制的历史记录
                                        </p>
                                    </div>
                                    <input
                                        id="auto-cleanup"
                                        type="checkbox"
                                        bind:checked={settings.autoCleanup}
                                        class="w-11 h-6 appearance-none bg-gray-200 dark:bg-gray-700 rounded-full relative cursor-pointer transition-colors
                                               checked:bg-primary
                                               before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                                               checked:before:translate-x-5"
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
                                        <p
                                            class="text-xs text-muted-foreground"
                                        >
                                            开启后保存原图，占用更多空间
                                        </p>
                                    </div>
                                    <input
                                        id="store-original-image"
                                        type="checkbox"
                                        bind:checked={
                                            settings.storeOriginalImage
                                        }
                                        class="w-11 h-6 appearance-none bg-gray-200 dark:bg-gray-700 rounded-full relative cursor-pointer transition-colors
                                               checked:bg-primary
                                               before:content-[''] before:absolute before:top-1 before:left-1 before:w-4 before:h-4 before:bg-white checked:before:bg-primary-foreground before:rounded-full before:transition-transform
                                               checked:before:translate-x-5"
                                    />
                                </div>
                            </Card>
                        </div>
                    {:else if activeTab === "tray"}
                        <!-- 托盘菜单设置 -->
                        <div
                            class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300"
                        >
                            <div>
                                <h2 class="text-lg font-semibold mb-1">
                                    托盘菜单
                                </h2>
                                <p class="text-sm text-muted-foreground">
                                    自定义系统托盘菜单的显示内容
                                </p>
                            </div>

                            <Card class="p-6 space-y-6">
                                <div class="space-y-4">
                                    <div class="flex justify-between">
                                        <label
                                            for="tray-text-length"
                                            class="text-sm font-medium"
                                            >文本长度限制</label
                                        >
                                        <span
                                            class="text-sm font-bold text-primary"
                                            >{settings.trayTextLength} 字符</span
                                        >
                                    </div>
                                    <input
                                        id="tray-text-length"
                                        type="range"
                                        min="15"
                                        max="80"
                                        step="5"
                                        bind:value={settings.trayTextLength}
                                        class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                                    />
                                </div>

                                <div class="space-y-4">
                                    <div class="flex justify-between">
                                        <label
                                            for="max-pinned-tray"
                                            class="text-sm font-medium"
                                            >最多置顶项</label
                                        >
                                        <span
                                            class="text-sm font-bold text-primary"
                                            >{settings.maxPinnedInTray} 条</span
                                        >
                                    </div>
                                    <input
                                        id="max-pinned-tray"
                                        type="range"
                                        min="3"
                                        max="10"
                                        step="1"
                                        bind:value={settings.maxPinnedInTray}
                                        class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                                    />
                                </div>

                                <div class="space-y-4">
                                    <div class="flex justify-between">
                                        <label
                                            for="max-recent-tray"
                                            class="text-sm font-medium"
                                            >最多最近项</label
                                        >
                                        <span
                                            class="text-sm font-bold text-primary"
                                            >{settings.maxRecentInTray} 条</span
                                        >
                                    </div>
                                    <input
                                        id="max-recent-tray"
                                        type="range"
                                        min="10"
                                        max="50"
                                        step="5"
                                        bind:value={settings.maxRecentInTray}
                                        class="w-full accent-primary h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                                    />
                                </div>
                            </Card>
                        </div>
                    {:else if activeTab === "storage"}
                        <!-- 数据存储设置 -->
                        <div
                            class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300"
                        >
                            <div>
                                <h2 class="text-lg font-semibold mb-1">
                                    数据存储
                                </h2>
                                <p class="text-sm text-muted-foreground">
                                    管理应用数据的存储位置
                                </p>
                            </div>

                            <Card class="p-6 space-y-6">
                                <div class="space-y-3">
                                    <label
                                        for="current-data-path"
                                        class="text-sm font-medium"
                                        >当前存储位置</label
                                    >
                                    <div class="flex items-center gap-2">
                                        <Input
                                            id="current-data-path"
                                            value={currentDataPath}
                                            readonly
                                            class="flex-1 text-xs font-mono"
                                        />
                                        <Button
                                            type="button"
                                            variant="outline"
                                            size="sm"
                                            onclick={openDataFolder}
                                        >
                                            打开
                                        </Button>
                                    </div>
                                    <p class="text-xs text-muted-foreground">
                                        包含数据库、加密密钥等重要文件
                                    </p>
                                </div>

                                <div class="pt-2 border-t border-border">
                                    <div
                                        class="flex items-center justify-between"
                                    >
                                        <div class="space-y-0.5">
                                            <span class="text-sm font-medium"
                                                >迁移数据</span
                                            >
                                            <p
                                                class="text-xs text-muted-foreground"
                                            >
                                                将数据移动到新的存储位置
                                            </p>
                                        </div>
                                        <Button
                                            type="button"
                                            variant="secondary"
                                            onclick={changeDataLocation}
                                            disabled={changingDataPath}
                                        >
                                            {#if changingDataPath}
                                                <Loader2
                                                    class="h-4 w-4 animate-spin mr-2"
                                                /> 处理中...
                                            {:else}
                                                更改位置...
                                            {/if}
                                        </Button>
                                    </div>
                                </div>
                            </Card>
                        </div>
                    {:else if activeTab === "about"}
                        <!-- 关于和更新 -->
                        <div
                            class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300"
                        >
                            <div>
                                <h2 class="text-lg font-semibold mb-1">关于</h2>
                                <p class="text-sm text-muted-foreground">
                                    版本信息和软件更新
                                </p>
                            </div>

                            <Card class="p-6 space-y-6">
                                <div class="flex items-center gap-4">
                                    <div
                                        class="h-12 w-12 bg-primary/10 rounded-xl flex items-center justify-center"
                                    >
                                        <Info class="h-6 w-6 text-primary" />
                                    </div>
                                    <div>
                                        <h3 class="font-bold text-lg">
                                            ClipMan
                                        </h3>
                                        <p
                                            class="text-sm text-muted-foreground"
                                        >
                                            高效的剪贴板管理工具
                                        </p>
                                        <div
                                            class="flex items-center gap-2 mt-1"
                                        >
                                            <span
                                                class="text-xs bg-muted px-2 py-0.5 rounded text-muted-foreground"
                                                >v1.7.0</span
                                            >
                                            <a
                                                href="https://github.com/Kiaana/ClipMan"
                                                target="_blank"
                                                class="text-xs text-primary hover:underline"
                                                >GitHub 仓库</a
                                            >
                                        </div>
                                    </div>
                                </div>

                                <div
                                    class="space-y-4 pt-4 border-t border-border"
                                >
                                    {#if updateInfo}
                                        <div class="space-y-2">
                                            <div
                                                class="flex justify-between text-sm"
                                            >
                                                <span
                                                    class="text-muted-foreground"
                                                    >当前版本</span
                                                >
                                                <span class="font-mono"
                                                    >{updateInfo.current_version}</span
                                                >
                                            </div>

                                            {#if updateInfo.available && updateInfo.latest_version}
                                                <div
                                                    class="flex justify-between text-sm"
                                                >
                                                    <span
                                                        class="text-muted-foreground"
                                                        >最新版本</span
                                                    >
                                                    <span
                                                        class="font-mono font-bold text-green-600 dark:text-green-400"
                                                        >{updateInfo.latest_version}</span
                                                    >
                                                </div>

                                                {#if updateInfo.body}
                                                    <div
                                                        class="mt-3 p-3 bg-muted/50 rounded border border-border text-sm"
                                                    >
                                                        <strong
                                                            class="block mb-1 text-xs uppercase tracking-wider text-muted-foreground"
                                                            >更新内容</strong
                                                        >
                                                        <pre
                                                            class="whitespace-pre-wrap font-sans text-sm">{updateInfo.body}</pre>
                                                    </div>
                                                {/if}
                                            {/if}
                                        </div>
                                    {:else}
                                        <div
                                            class="text-center py-4 text-sm text-muted-foreground"
                                        >
                                            点击检查更新获取最新版本信息
                                        </div>
                                    {/if}

                                    <div class="flex gap-2 pt-2">
                                        <Button
                                            type="button"
                                            variant="secondary"
                                            class="flex-1"
                                            onclick={checkForUpdates}
                                            disabled={checkingUpdate ||
                                                installingUpdate}
                                        >
                                            {#if checkingUpdate}
                                                <Loader2
                                                    class="h-4 w-4 animate-spin mr-2"
                                                /> 检查中...
                                            {:else}
                                                <RefreshCw
                                                    class="h-4 w-4 mr-2"
                                                /> 检查更新
                                            {/if}
                                        </Button>

                                        {#if updateInfo?.available}
                                            <Button
                                                type="button"
                                                class="flex-1 bg-green-600 hover:bg-green-700 text-white"
                                                onclick={installUpdate}
                                                disabled={installingUpdate}
                                            >
                                                {#if installingUpdate}
                                                    <Loader2
                                                        class="h-4 w-4 animate-spin mr-2"
                                                    /> 安装中...
                                                {:else}
                                                    <Download
                                                        class="h-4 w-4 mr-2"
                                                    /> 安装更新
                                                {/if}
                                            </Button>
                                        {/if}
                                    </div>

                                    {#if updateMessage}
                                        <div
                                            class="p-3 rounded text-sm text-center
                                            {updateMessage.includes('失败')
                                                ? 'bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-200'
                                                : updateMessage.includes(
                                                        '最新版本',
                                                    ) ||
                                                    updateMessage.includes(
                                                        '成功',
                                                    )
                                                  ? 'bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-200'
                                                  : 'bg-muted text-muted-foreground'}"
                                        >
                                            {updateMessage}
                                        </div>
                                    {/if}
                                </div>
                            </Card>
                        </div>
                    {/if}
                </div>
            {/if}
        </main>
    </div>

    {#if message}
        <div
            class="absolute bottom-6 right-6 p-4 rounded-md shadow-lg text-sm font-medium animate-in slide-in-from-bottom-4 fade-in duration-300 z-50
            {message.includes('失败')
                ? 'bg-destructive text-destructive-foreground'
                : 'bg-primary text-primary-foreground'}"
        >
            {message}
        </div>
    {/if}
</div>

<!-- 数据迁移确认对话框 -->
{#if showMigrationDialog}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 backdrop-blur-sm"
    >
        <div
            class="bg-background border border-border rounded-lg p-6 max-w-lg w-full mx-4 space-y-4 shadow-xl animate-in zoom-in-95 duration-200"
        >
            <h3 class="text-lg font-semibold">确认数据迁移</h3>

            <div class="space-y-2 text-sm">
                <div>
                    <span class="font-medium">当前位置:</span>
                    <p
                        class="font-mono text-xs bg-muted/50 text-foreground p-2 rounded mt-1 break-all border border-border"
                    >
                        {currentDataPath}
                    </p>
                </div>
                <div>
                    <span class="font-medium">新位置:</span>
                    <p
                        class="font-mono text-xs bg-muted/50 text-foreground p-2 rounded mt-1 break-all border border-border"
                    >
                        {newDataPath}
                    </p>
                </div>
            </div>

            <div
                class="flex items-start gap-2 p-3 bg-yellow-50 dark:bg-yellow-950/50 rounded border border-yellow-200 dark:border-yellow-800"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="text-yellow-700 dark:text-yellow-500 flex-shrink-0"
                    ><path
                        d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"
                    /><line x1="12" x2="12" y1="9" y2="13" /><line
                        x1="12"
                        x2="12.01"
                        y1="17"
                        y2="17"
                    /></svg
                >
                <p class="text-sm text-yellow-900 dark:text-yellow-300">
                    数据迁移过程中，剪贴板监控将暂时停止。迁移完成后会自动恢复。
                </p>
            </div>

            <div class="flex items-center gap-2">
                <input
                    type="checkbox"
                    id="delete-old-data"
                    bind:checked={deleteOldData}
                    class="w-4 h-4 rounded border-input text-primary focus:ring-2 focus:ring-ring"
                />
                <label
                    for="delete-old-data"
                    class="text-sm font-medium cursor-pointer"
                >
                    迁移完成后删除旧位置的数据
                </label>
            </div>

            <div class="flex gap-2 justify-end">
                <Button
                    type="button"
                    variant="outline"
                    onclick={() => (showMigrationDialog = false)}
                    disabled={changingDataPath}
                >
                    取消
                </Button>
                <Button
                    type="button"
                    onclick={confirmMigration}
                    disabled={changingDataPath}
                >
                    {#if changingDataPath}
                        <Loader2 class="h-4 w-4 animate-spin mr-2" /> 迁移中...
                    {:else}
                        确认迁移
                    {/if}
                </Button>
            </div>
        </div>
    </div>
{/if}
