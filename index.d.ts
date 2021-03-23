declare module "globkey" {
    export function on(callback: (action: string, key: string) => void): void;
    export function raw(callback: (keypair: [string, string]) => void): void;
    export function capturecombo(): [string, string];
}