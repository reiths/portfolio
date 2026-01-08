<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { getCurrentWebview } from "@tauri-apps/api/webview";

    export let onDrop: (paths: string[]) => Promise<void> = async () => {};

    let isDragOver = false;
    let unlisten: (() => void) | null = null;

    onMount(async () => {
        unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
            switch (event.payload.type) {
                case "over":
                    isDragOver = true;
                    break;
                case "drop":
                    isDragOver = false;
                    try {
                        await onDrop(event.payload.paths);
                    } catch (error) {
                        console.error("Error processing files:", error);
                    }
                    break;
                default:
                    isDragOver = false;
            }
        });
    });

    onDestroy(() => {
        if (unlisten) {
            unlisten();
        }
    });
</script>

<div class="drop-zone" class:drag-over={isDragOver}>
    <slot>Drag and drop files here</slot>
</div>

<style>
    .drop-zone {
        border: 2px dashed #ccc;
        border-radius: 8px;
        padding: 3rem;
        text-align: center;
        background-color: #f9f9f9;
        cursor: pointer;
        transition: all 0.3s ease;
    }

    .drop-zone.drag-over {
        border-color: #4caf50;
        background-color: #e8f5e9;
        box-shadow: 0 0 10px rgba(76, 175, 80, 0.3);
    }
</style>
