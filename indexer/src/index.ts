import { ponder } from "ponder:registry";
import {
    placeOrderEvents,
    insertOrderEvents,
    updateOrderEvents,
    setTickDataEvents,
    setCurrentTickEvents,
    flipTickEvents
} from "ponder:schema";

ponder.on("Engine:PlaceOrder", async ({ event, context }) => {
    await context.db.insert(placeOrderEvents).values({
        id: Number(event.log.id),
        user: event.args.user,
        tick: event.args.tick,
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("OrderManager:InsertOrder", async ({ event, context }) => {
    await context.db.insert(insertOrderEvents).values({
        id: Number(event.log.id),
        user: event.args.user,
        tick: event.args.tick,
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("OrderManager:UpdateOrder", async ({ event, context }) => {
    await context.db.insert(updateOrderEvents).values({
        id: Number(event.log.id),
        tick: event.args.tick,
        order_index: event.args.order_index,
        volume: event.args.volume,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("TickManager:SetTickData", async ({ event, context }) => {
    await context.db.insert(setTickDataEvents).values({
        id: Number(event.log.id),
        tick: event.args.tick,
        is_buy: event.args.is_buy,
        volume: event.args.volume,
        is_existing_order: event.args.is_existing_order,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("BitmapManager:SetCurrentTick", async ({ event, context }) => {
    await context.db.insert(setCurrentTickEvents).values({
        id: Number(event.log.id),
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    });
});

ponder.on("BitmapManager:FlipTick", async ({ event, context }) => {
    await context.db.insert(flipTickEvents).values({
        id: Number(event.log.id),
        tick: event.args.tick,
        timestamp: Number(event.block.timestamp),
    });
});