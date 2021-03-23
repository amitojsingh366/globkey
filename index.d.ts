declare module "globkey" {
    export function on(callback: (action: string, key: number) => void): void;
    export function raw(callback: (keypair: [number, number]) => void): void;
    export function capturecombo(): [number, number];
}