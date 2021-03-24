declare module "globkey" {
    export function raw(callback: (keypair: string[]) => void): void;
    export function capturecombo(): [string, string];
}