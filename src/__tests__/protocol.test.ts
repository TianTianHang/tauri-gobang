import { describe, it, expect } from "vitest";
import type { NetworkMessage, ServerMessage, AuthResponse, RoomListEntry } from "../types/game";

describe("NetworkMessage serialization", () => {
  it("serializes move message", () => {
    const msg: NetworkMessage = { type: "move", row: 7, col: 8 };
    const json = JSON.stringify(msg);
    const parsed = JSON.parse(json);
    expect(parsed.type).toBe("move");
    expect(parsed.row).toBe(7);
    expect(parsed.col).toBe(8);
  });

  it("serializes restart_request", () => {
    const msg: NetworkMessage = { type: "restart_request" };
    const json = JSON.stringify(msg);
    const parsed = JSON.parse(json);
    expect(parsed.type).toBe("restart_request");
    expect(parsed.row).toBeUndefined();
  });

  it("serializes restart_accept", () => {
    const msg: NetworkMessage = { type: "restart_accept" };
    const json = JSON.stringify(msg);
    expect(json).toContain('"restart_accept"');
  });

  it("serializes disconnect", () => {
    const msg: NetworkMessage = { type: "disconnect" };
    const json = JSON.stringify(msg);
    expect(json).toContain('"disconnect"');
  });
});

describe("ServerMessage deserialization", () => {
  it("parses game_start message", () => {
    const raw = '{"type":"game_start","black_player":"Alice","white_player":"Bob"}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.type).toBe("game_start");
    expect(msg.black_player).toBe("Alice");
    expect(msg.white_player).toBe("Bob");
  });

  it("parses opponent_joined message", () => {
    const raw = '{"type":"opponent_joined","username":"Bob"}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.type).toBe("opponent_joined");
    expect(msg.username).toBe("Bob");
  });

  it("parses opponent_disconnected message", () => {
    const raw = '{"type":"opponent_disconnected","username":"Bob","can_reconnect":true,"timeout_seconds":30}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.type).toBe("opponent_disconnected");
    expect(msg.can_reconnect).toBe(true);
    expect(msg.timeout_seconds).toBe(30);
  });

  it("parses player_reconnected message", () => {
    const raw = '{"type":"player_reconnected","username":"Bob"}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.type).toBe("player_reconnected");
  });

  it("parses game_ended with winner", () => {
    const raw = '{"type":"game_ended","winner":"Alice","reason":"opponent_disconnected"}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.type).toBe("game_ended");
    expect(msg.winner).toBe("Alice");
    expect(msg.reason).toBe("opponent_disconnected");
  });

  it("parses game_ended with no winner (draw)", () => {
    const raw = '{"type":"game_ended","winner":null,"reason":"both_disconnected"}';
    const msg: ServerMessage = JSON.parse(raw);
    expect(msg.winner).toBeNull();
  });
});

describe("AuthResponse type", () => {
  it("has expected shape", () => {
    const resp: AuthResponse = {
      token: "uuid-token",
      user_id: "user-123",
      username: "testuser",
    };
    expect(resp.token).toBeDefined();
    expect(resp.user_id).toBeDefined();
    expect(resp.username).toBeDefined();
  });
});

describe("RoomListEntry type", () => {
  it("has expected shape", () => {
    const entry: RoomListEntry = {
      id: "room-1",
      name: "Test Room",
      host_username: "Alice",
      created_at: Date.now(),
      player_count: 1,
      status: "waiting",
    };
    expect(entry.player_count).toBe(1);
    expect(entry.status).toBe("waiting");
    expect(entry.id).toBeDefined();
    expect(entry.name).toBeDefined();
    expect(entry.host_username).toBeDefined();
    expect(typeof entry.created_at).toBe("number");
  });
});
