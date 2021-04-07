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
     * @returns an array of the currently pressed keys, or a boolean of false if it failed.
     * @remarks this method will only fucntion properly if the `start()` function has already been called
     */
    export function getKeys(): string[] | boolean;
    /**
     * @returns a boolean value based on if globkey is runnig or not.
     * @remarks this method will only return `true` if `start()` has been called
     */
    export function isRunning(): boolean;
}