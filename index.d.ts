declare module "globkey" {
    export function raw(callback: (keypair: string[]) => void): Promise<boolean>;
    export function unload(): void;
    export function get_keys(): string[];
}