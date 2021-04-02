declare module "globkey" {
    export function raw(callback: (keypair: string[]) => void): boolean;
    export function stop(): void;
    export function unload(): void;
}