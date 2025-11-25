<script lang="ts">
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import { router } from "$lib/stores/router.svelte";
  import { themeStore } from "$lib/stores/theme.svelte";
  import { i18n } from "$lib/i18n";
  import SearchBar from "$lib/components/SearchBar.svelte";
  import ClipboardItem from "$lib/components/ClipboardItem.svelte";
  import SettingsPage from "./settings/+page.svelte";
  import PermissionCheck from "$lib/components/PermissionCheck.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import {
    Sun,
    Moon,
    Monitor,
    Settings,
    Trash2,
    Pin,
    ClipboardList,
    FileText,
    Image as ImageIcon,
    Loader2,
    Heart,
  } from "lucide-svelte";
  import { flip } from "svelte/animate";

  // Reactive reference to translations
  const t = $derived(i18n.t);

  // Reactive state showing pinned vs all
  let showPinned = $state(false);

  // Derived: current list to display
  const displayItems = $derived(
    showPinned ? clipboardStore.pinnedItems : clipboardStore.filteredItems,
  );

  async function clearHistory() {
    if (confirm(t.confirmClearHistory)) {
      await clipboardStore.clearNonPinned();
    }
  }

  $effect(() => {
    const theme = themeStore.current;
    const root = document.documentElement;
    const isDark =
      theme === "dark" ||
      (theme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    // Remove all theme classes first
    root.classList.remove("dark", "light-pink");

    if (theme === "light-pink") {
      root.classList.add("light-pink");
    } else if (isDark) {
      root.classList.add("dark");
    }

    localStorage.setItem("theme", theme);
  });
</script>

{#if router.currentRoute === "settings"}
  <SettingsPage />
{:else}
  <div
    class="flex flex-col h-screen bg-background text-foreground overflow-hidden"
  >
    <PermissionCheck />
    <Toast />

    <header
      class="flex-none p-4 border-b border-border bg-muted/30 backdrop-blur-sm sticky top-0 z-10"
    >
      <div class="flex items-center justify-between mb-4">
        <h1 class="text-2xl font-bold tracking-tight">{t.appName}</h1>
        <div class="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            onclick={() => themeStore.toggle()}
            title={t.switchTheme}
          >
            {#if themeStore.current === "light"}
              <Sun class="h-4 w-4" />
            {:else if themeStore.current === "dark"}
              <Moon class="h-4 w-4" />
            {:else if themeStore.current === "light-pink"}
              <Heart class="h-4 w-4" />
            {:else}
              <Monitor class="h-4 w-4" />
            {/if}
          </Button>
          <Button
            variant="ghost"
            size="icon"
            title={t.settings}
            onclick={() => router.goToSettings()}
          >
            <Settings class="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div class="flex items-center gap-2 mb-4">
        <div class="flex p-1 bg-muted rounded-lg" role="tablist">
          <button
            role="tab"
            aria-selected={!showPinned}
            aria-controls="clipboard-content"
            tabindex={showPinned ? -1 : 0}
            class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {!showPinned
              ? 'bg-background text-foreground shadow-sm'
              : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (showPinned = false)}
          >
            {t.history}
          </button>
          <button
            role="tab"
            aria-selected={showPinned}
            aria-controls="clipboard-content"
            tabindex={showPinned ? 0 : -1}
            class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {showPinned
              ? 'bg-background text-foreground shadow-sm'
              : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (showPinned = true)}
          >
            {t.pinned} <span class="ml-1 text-xs opacity-70"
              >({clipboardStore.pinnedItems.length})</span
            >
          </button>
        </div>

        <div class="ml-auto">
          <Button
            variant="ghost"
            size="icon"
            title={t.clearNonPinned}
            onclick={clearHistory}
            class="text-muted-foreground hover:text-destructive"
          >
            <Trash2 class="h-4 w-4" />
          </Button>
        </div>
      </div>

      <SearchBar />
    </header>

    <main
      id="clipboard-content"
      class="flex-1 overflow-hidden flex flex-col bg-background"
    >
      {#if clipboardStore.isLoading}
        <div
          class="flex flex-col items-center justify-center h-full text-muted-foreground"
        >
          <Loader2 class="h-8 w-8 animate-spin mb-2" />
          <p>{t.loading}</p>
        </div>
      {:else if displayItems.length === 0}
        <div
          class="flex flex-col items-center justify-center h-full text-muted-foreground text-center p-4"
        >
          {#if showPinned}
            <Pin class="h-12 w-12 mb-4 opacity-20" />
            <p class="font-medium">{t.noPinnedItems}</p>
            <p class="text-sm mt-1 opacity-70">{t.noPinnedItemsHint}</p>
          {:else}
            <ClipboardList class="h-12 w-12 mb-4 opacity-20" />
            <p class="font-medium">{t.noClipboardHistory}</p>
            <p class="text-sm mt-1 opacity-70">{t.noClipboardHistoryHint}</p>
            <div
              class="mt-8 p-4 bg-muted/50 rounded-lg text-xs text-left space-y-1 max-w-xs mx-auto"
            >
              <p class="font-semibold mb-2 opacity-70">{t.statistics}</p>
              <div class="flex justify-between">
                <span>{t.total}:</span> <span>{clipboardStore.items.length}</span>
              </div>
              <div class="flex justify-between">
                <span>{t.text}:</span>
                <span
                  >{clipboardStore.items.filter((i) => i.contentType === "text")
                    .length}</span
                >
              </div>
              <div class="flex justify-between">
                <span>{t.image}:</span>
                <span
                  >{clipboardStore.items.filter(
                    (i) => i.contentType === "image",
                  ).length}</span
                >
              </div>
            </div>
          {/if}
        </div>
      {:else}
        <div
          class="flex-none px-4 py-2 flex justify-between items-center text-xs text-muted-foreground border-b border-border/50 bg-background/95 backdrop-blur z-10"
        >
          <span>{t.showing} {displayItems.length} {t.items}</span>
          <div class="flex gap-2 items-center">
            <span class="flex items-center gap-1"
              ><FileText class="h-3 w-3" />
              {displayItems.filter((i) => i.contentType === "text")
                .length}</span
            >
            <span class="flex items-center gap-1"
              ><ImageIcon class="h-3 w-3" />
              {displayItems.filter((i) => i.contentType === "image")
                .length}</span
            >
          </div>
        </div>

        <div class="flex-1 overflow-y-auto p-4 pb-8 space-y-2">
          {#each displayItems as item (item.id)}
            <div animate:flip={{ duration: 300 }}>
              <ClipboardItem {item} />
            </div>
          {/each}
        </div>
      {/if}
    </main>
  </div>
{/if}
