import { invoke } from '@tauri-apps/api/core';

// ===== 游戏 =====

export async function addGame(name: string, icon?: string) {
  return await invoke('add_game', { name, icon });
}

export async function getGames() {
  return await invoke('get_games');
}

export async function addGameRecord(
  gameId: string,
  recordType: string,
  purchaseDate?: string,
  startDate?: string,
  expireDate?: string,
  amount?: number,
  note?: string
) {
  return await invoke('add_game_record', {
    gameId,
    recordType,
    purchaseDate,
    startDate,
    expireDate,
    amount,
    note,
  });
}

export async function getGameRecords(gameId: string) {
  return await invoke('get_game_records', { gameId });
}

// ===== 硬件 =====

export async function getHardwareTypes() {
  return await invoke('get_hardware_types');
}

export async function addHardware(
  typeId: string,
  name: string,
  brand?: string,
  purchaseDate?: string,
  purchaseChannel?: string,
  warrantyMonths?: number,
  price?: number,
  orderNo?: string,
  note?: string
) {
  return await invoke('add_hardware', {
    typeId,
    name,
    brand,
    purchaseDate,
    purchaseChannel,
    warrantyMonths,
    price,
    orderNo,
    note,
  });
}

export async function getHardwares() {
  return await invoke('get_hardwares');
}

// ===== 耗材 =====

export async function getConsumableTypes() {
  return await invoke('get_consumable_types');
}

export async function getConsumables() {
  return await invoke('get_consumables');
}

// ===== 药品 =====

export async function getMedicineCategories() {
  return await invoke('get_medicine_categories');
}

export async function getMedicines() {
  return await invoke('get_medicines');
}

// ===== 仪表盘 =====

export async function getDashboardStats() {
  return await invoke('get_dashboard_stats');
}

// ===== 初始化 =====

export async function initDb() {
  return await invoke('init_db');
}
