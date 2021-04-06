declare module "globkey" {
    /**
     * Loads and starts the globkey library
     */
    export function start(): void;
    /**
     * Unloads and stops the globkey library
     */
    export function unload(): void;
    /**
     * @returns an array of the currently pressed keys.
     * @remarks this method will only fucntion properly if the `start()` function has already been called
     */
    export function getKeys(): string[];
}