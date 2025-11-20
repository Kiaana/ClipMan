<script lang="ts" generics="T extends { id: string }">
    interface Props {
        items: T[];
        estimateHeight?: number;
        children: import("svelte").Snippet<[T]>;
        class?: string;
    }

    let {
        items,
        estimateHeight = 100,
        children,
        class: className = "",
    }: Props = $props();

    let container: HTMLDivElement;
    let scrollTop = $state(0);
    let clientHeight = $state(0);

    // Buffer items to render above/below view
    const OVERSCAN = 5;

    const startIndex = $derived(
        Math.max(0, Math.floor(scrollTop / estimateHeight) - OVERSCAN),
    );
    const endIndex = $derived(
        Math.min(
            items.length,
            Math.ceil((scrollTop + clientHeight) / estimateHeight) + OVERSCAN,
        ),
    );

    const visibleItems = $derived(items.slice(startIndex, endIndex));

    const paddingTop = $derived(startIndex * estimateHeight);
    const paddingBottom = $derived(
        Math.max(0, (items.length - endIndex) * estimateHeight),
    );

    function handleScroll(e: Event) {
        const target = e.target as HTMLDivElement;
        scrollTop = target.scrollTop;
    }
</script>

<div
    bind:this={container}
    bind:clientHeight
    onscroll={handleScroll}
    class="overflow-y-auto h-full {className}"
>
    <div
        style="padding-top: {paddingTop}px; padding-bottom: {paddingBottom}px;"
    >
        {#each visibleItems as item (item.id)}
            {@render children(item)}
        {/each}
    </div>
</div>
