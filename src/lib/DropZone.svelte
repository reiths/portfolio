<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { getCurrentWebview } from "@tauri-apps/api/webview";
    import { open } from "@tauri-apps/plugin-dialog";

    export let onDrop: (paths: string[]) => Promise<void> = async () => {};

    let isDragOver = false;
    let unlisten: (() => void) | null = null;

    async function selectFiles() {
        const result = await open({ multiple: true });
        if (!result) return;
        const paths = Array.isArray(result) ? result : [result];
        await onDrop(paths as string[]);
    }

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

<button
    type="button"
    class="drop-zone"
    class:drag-over={isDragOver}
    on:click={selectFiles}
>
    <slot>Drag and drop files here (or click to select)</slot>
</button>

<style>
    .drop-zone {
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        border: 2px dashed #ccc;
        border-radius: 8px;
        padding: 3rem;
        text-align: center;
        background-color: #f9f9f9;
        cursor: pointer;
        transition: all 0.3s ease;
        display: inline-block;
        width: 100%;
    }

    .drop-zone:focus {
        outline: 3px solid rgba(33, 150, 243, 0.6);
        outline-offset: 2px;
    }

    .drop-zone.drag-over {
        border-color: #4caf50;
        background-color: #e8f5e9;
        box-shadow: 0 0 10px rgba(76, 175, 80, 0.3);
    }
</style>
