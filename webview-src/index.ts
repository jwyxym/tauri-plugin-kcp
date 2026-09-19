import { invoke } from '@tauri-apps/api/core';
import { EventCallback, Options, listen as _listen } from '@tauri-apps/api/event';
import { Buffer } from 'buffer';

/**
 * 
 * @param id A unique ID
 * @param remote e.g. 127.0.0.1:8080
 */
export async function connect(id: string, remote: string) {
  await invoke('plugin:kcp|connect', {
    id, remote,
  });
}

export async function server(id: string, bindAt: string) {
  await invoke('plugin:kcp|listen', {
    id, bindAt,
  });
}

export async function close(id: string) {
  await invoke('plugin:kcp|close', {
    id,
  });
}

/**
 * 
 * @param id A unique ID
 * @param message A string or a uint8 array
 */
export async function send(id: string, message: string | number[]) {
  await invoke('plugin:kcp|send', {
    id,
    message: typeof message === 'string' ? Array.from(Buffer.from(message)) : message,
  });
}

export interface Payload {
  id: string;
  addr: string;
  data: number[];
}

export function listen(handler: EventCallback<Payload>, options?: Options) {
  return _listen('plugin://kcp', handler, options);
}
