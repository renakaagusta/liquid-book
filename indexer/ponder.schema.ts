import { onchainTable } from "ponder";

export const placeOrderEvents = onchainTable("place_order_events", (t) => ({
  id: t.text().primaryKey(),
  user: t.hex(),
  tick: t.bigint(),
  order_index: t.bigint(),
  is_buy: t.boolean(),
  is_market: t.boolean(),
  volume: t.bigint(),
  remaining_volume: t.bigint(),
  timestamp: t.integer().notNull(),
}));

export const insertOrderEvents = onchainTable("insert_order_events", (t) => ({
  id: t.text().primaryKey(),
  user: t.hex(),
  tick: t.bigint(),
  order_index: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
  timestamp: t.integer().notNull(),
}));

export const updateOrderEvents = onchainTable("update_order_events", (t) => ({
  id: t.text().primaryKey(),
  tick: t.bigint(),
  order_index: t.bigint(),
  volume: t.bigint(),
  timestamp: t.integer().notNull(),
}));

export const setTickDataEvents = onchainTable("set_tick_data_events", (t) => ({
  id: t.text().primaryKey(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
  is_existing_order: t.boolean(),
  timestamp: t.integer().notNull(),
}));

export const setCurrentTickEvents = onchainTable("set_current_tick_events", (t) => ({
  id: t.text().primaryKey(),
  tick: t.bigint(),
  timestamp: t.integer().notNull(),
}));

export const flipTickEvents = onchainTable("set_flip_tick_events", (t) => ({
  id: t.text().primaryKey(),
  tick: t.integer(),
  timestamp: t.integer().notNull(),
}));

export const ticks = onchainTable("ticks", (t) => ({
  id: t.text().primaryKey(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
  timestamp: t.integer().notNull(),
}));

export const orders = onchainTable("orders", (t) => ({
  id: t.text().primaryKey(),
  user: t.hex(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  is_market: t.boolean(),
  is_filled: t.boolean(),
  volume: t.bigint(),
  filled_volume: t.bigint(),
  timestamp: t.integer().notNull(),
}));