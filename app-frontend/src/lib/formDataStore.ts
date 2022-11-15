import { writable } from "svelte/store";

export const postType = writable<number | null>(null);
export const categoryType = writable<string | null>(null);