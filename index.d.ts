declare module "globkey" {
    export function start(): void;
    export function unload(): void;
    export function getKeys(): string[];
}