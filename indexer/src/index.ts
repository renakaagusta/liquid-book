import { ponder } from "ponder:registry";
import {
    placeOrderEvents,
    insertOrderEvents,
    updateOrderEvents,
    setTickDataEvents,
    setCurrentTickEvents,
    flipTickEvents,
    ticks,
    orders
} from "ponder:schema";

ponder.on("Engine:PlaceOrder", async ({ event, context }) => {
    await context.db
        .insert(orders)
        .values({
            id: String(event.transaction.hash),
            user: event.args.user,
            tick: event.args.tick,
            order_index: event.args.order_index,
            is_buy: event.args.is_buy,
            is_market: event.args.is_market,
            volume: event.args.volume,
            remaining_volume: event.args.remaining_volume,
            timestamp: Number(event.block.timestamp),
        });

    await context.db.insert(placeOrderEvents).values({
        id: event.transaction.hash,
        user: event.args.user,
        tick: event.args.tick,
        order_index: event.args.order_index,
        is_buy: event.args.is_buy,
        is_market: event.args.is_market,
        volume: event.args.volume,
        remaining_volume: event.args.remaining_volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("OrderManager:InsertOrder", async ({ event, context }) => {
    await context.db.insert(insertOrderEvents).values({
        id: event.transaction.hash,
        user: event.args.user,
        tick: event.args.tick,
        order_index: event.args.order_index,
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("OrderManager:UpdateOrder", async ({ event, context }) => {
    const row = await context.db.find(orders, { tick: event.args.tick, order_index: event.args.order_index, is_filled: false });

    if (row) {
        await context.db
            .update(orders, { id: row.id })
            .set({
                volume: event.args.volume,
                is_filled: Number(event.args.volume) == 0,
                timestamp: Number(event.block.timestamp),
            });
    }

    await context.db.insert(updateOrderEvents).values({
        id: event.transaction.hash,
        tick: event.args.tick,
        order_index: event.args.order_index,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    }).onConflictDoUpdate({
        tick: event.args.tick,
        order_index: event.args.order_index,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("TickManager:SetTickData", async ({ event, context }) => {
    await context.db
        .insert(ticks)
        .values({
            id: String(event.args.tick),
            tick: event.args.tick,
            is_buy: event.args.is_buy,
            volume: event.args.volume,
            timestamp: Number(event.block.timestamp),
        }).onConflictDoUpdate({
            is_buy: event.args.is_buy,
            volume: event.args.volume,
            timestamp: Number(event.block.timestamp),
        });

    await context.db.insert(setTickDataEvents).values({
        id: event.transaction.hash,
        tick: event.args.tick,
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        is_existing_order: event.args.is_existing_order,
        timestamp: Number(event.block.timestamp),
    }).onConflictDoUpdate({
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("BitmapManager:SetCurrentTick", async ({ event, context }) => {
    await context.db.insert(setCurrentTickEvents).values({
        id: event.transaction.hash,
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    }).onConflictDoUpdate({
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("BitmapManager:FlipTick", async ({ event, context }) => {
    await context.db.insert(flipTickEvents).values({
        id: event.transaction.hash,
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    }).onConflictDoUpdate({
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    });
});