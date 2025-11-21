/**
 * LINE Messaging API webhook example that forwards user messages to Codex.
 *
 * This example sets up an Express server that accepts LINE webhook requests,
 * forwards text messages to a Codex agent, and replies with the agent's
 * response. It demonstrates how to combine LINE's SDK with the Codex TypeScript
 * SDK while keeping full type safety.
 */

import express, { Request, Response } from "express";
import {
  Client,
  ClientConfig,
  MessageEvent,
  MiddlewareConfig,
  TextEventMessage,
  TextMessage,
  WebhookEvent,
  middleware,
} from "@line/bot-sdk";

import { Codex } from "../src/codex";
import { CodexOptions } from "../src/codexOptions";
import type { Input, Thread } from "../src/thread";

/** Combined configuration required by the LINE SDK middleware and client. */
type LineConfiguration = MiddlewareConfig & ClientConfig;

export interface LineCodexBotConfig {
  /** LINE channel configuration used for signature validation and replies. */
  line: LineConfiguration;
  /** Optional Codex connection options (API key, base URL, etc.). */
  codex?: CodexOptions;
  /**
   * Instruction prompt injected into the first turn for every user.
   * Defaults to a systems-style instruction that asks Codex to answer with
   * concise, actionable steps.
   */
  instructionPrompt?: string;
}

interface ThreadState {
  thread: Thread;
  isPrimed: boolean;
}

const DEFAULT_INSTRUCTION =
  "You are Codex, an expert AI pair programmer supporting users on LINE. " +
  "Focus on actionable coding instructions, respond in Japanese when the user " +
  "writes in Japanese, and keep replies concise.";

/**
 * Manages the Codex conversation state for LINE users and handles webhook events.
 */
export class LineCodexBot {
  private readonly codex: Codex;
  private readonly instructionPrompt: string;
  private readonly lineClient: Client;
  private readonly lineConfig: LineConfiguration;
  private readonly threads = new Map<string, ThreadState>();

  constructor(config: LineCodexBotConfig) {
    this.lineConfig = config.line;
    this.codex = new Codex(config.codex);
    this.lineClient = new Client(config.line);
    this.instructionPrompt = config.instructionPrompt ?? DEFAULT_INSTRUCTION;
  }

  /** Returns LINE's Express middleware for signature verification. */
  middleware() {
    return middleware(this.lineConfig);
  }

  /**
   * Dispatches an incoming webhook event. Non-text events are safely ignored
   * to avoid LINE delivery warnings.
   */
  async handleEvent(event: WebhookEvent): Promise<void> {
    if (!this.isTextMessage(event)) {
      return;
    }

    const userId = event.source.userId;
    if (!userId) {
      // Group / room conversations do not include a stable user id; skip them.
      return;
    }

    const state = this.getThreadState(userId);
    const input = this.buildInput(event.message.text, state.isPrimed);

    try {
      const turn = await state.thread.run(input);
      state.isPrimed = true;

      await this.lineClient.replyMessage(event.replyToken, this.toTextMessage(turn.finalResponse));
    } catch (error) {
      console.error("Failed to handle Codex turn", error);

      await this.safeReply(
        event,
        "申し訳ありません。現在Codexからの応答を取得できませんでした。しばらくしてからお試しください。",
      );
    }
  }

  private getThreadState(userId: string): ThreadState {
    const existing = this.threads.get(userId);
    if (existing) {
      return existing;
    }

    const thread = this.codex.startThread();
    const state: ThreadState = { thread, isPrimed: false };
    this.threads.set(userId, state);
    return state;
  }

  private buildInput(message: string, isPrimed: boolean): Input {
    if (isPrimed) {
      return [{ type: "text", text: message }];
    }

    return [
      { type: "text", text: this.instructionPrompt },
      { type: "text", text: message },
    ];
  }

  private isTextMessage(event: WebhookEvent): event is MessageEvent<TextEventMessage> {
    return event.type === "message" && event.message.type === "text";
  }

  private toTextMessage(text: string): TextMessage {
    return { type: "text", text };
  }

  private async safeReply(event: MessageEvent<TextEventMessage>, text: string) {
    try {
      await this.lineClient.replyMessage(event.replyToken, this.toTextMessage(text));
    } catch (replyError) {
      console.error("Failed to send fallback reply", replyError);
    }
  }
}

function getEnvVar(key: string): string {
  const value = process.env[key];
  if (!value) {
    throw new Error(`Missing required environment variable: ${key}`);
  }
  return value;
}

async function main() {
  const bot = new LineCodexBot({
    line: {
      channelAccessToken: getEnvVar("LINE_CHANNEL_ACCESS_TOKEN"),
      channelSecret: getEnvVar("LINE_CHANNEL_SECRET"),
    },
    codex: {
      baseUrl: process.env.CODEX_BASE_URL,
      apiKey: process.env.CODEX_API_KEY,
    },
    instructionPrompt: process.env.CODEX_INSTRUCTION_PROMPT,
  });

  const app = express();
  app.post("/webhook", bot.middleware(), async (req: Request, res: Response) => {
    const events = (req.body?.events as WebhookEvent[] | undefined) ?? [];
    await Promise.all(events.map((event) => bot.handleEvent(event)));
    res.json({ ok: true } as const);
  });

  app.get("/healthz", (_req: Request, res: Response) => {
    res.json({ status: "healthy" } as const);
  });

  const port = Number(process.env.PORT ?? 3000);
  app.listen(port, () => {
    console.log(`LINE Codex bot listening on port ${port}`);
  });
}

if (require.main === module) {
  main().catch((error) => {
    console.error("Failed to start LINE Codex bot", error);
    process.exitCode = 1;
  });
}
