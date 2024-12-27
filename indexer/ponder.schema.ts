import { onchainTable } from "ponder";

export const placeOrderEvents = onchainTable("place_order_events", (t) => ({
  id: t.integer().primaryKey(),
  user: t.hex(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
}));

export const insertOrderEvents = onchainTable("insert_order_events", (t) => ({
  id: t.integer().primaryKey(),
  user: t.hex(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
}));

export const updateOrderEvents = onchainTable("update_order_events", (t) => ({
  id: t.integer().primaryKey(),
  tick: t.bigint(),
  order_index: t.bigint(),
  volume: t.bigint(),
}));

export const setTickDataEvents = onchainTable("set_tick_data_events", (t) => ({
  id: t.integer().primaryKey(),
  tick: t.bigint(),
  is_buy: t.boolean(),
  volume: t.bigint(),
  is_existing_order: t.boolean(),
}));

export const setCurrentTickEvents = onchainTable("set_current_tick_events", (t) => ({
  id: t.integer().primaryKey(),
  tick: t.bigint(),
}));

export const flipTickEvents = onchainTable("set_flip_tick_events", (t) => ({
  id: t.integer().primaryKey(),
  tick: t.integer(),
}));
