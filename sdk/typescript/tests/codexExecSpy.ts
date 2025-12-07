import * as child_process from "node:child_process";

jest.mock("node:child_process", () => {
  const actual = jest.requireActual<typeof import("node:child_process")>("node:child_process");
  return { ...actual, spawn: jest.fn(actual.spawn) };
});

const actualChildProcess =
  jest.requireActual<typeof import("node:child_process")>("node:child_process");
const spawnMock = child_process.spawn as jest.MockedFunction<typeof actualChildProcess.spawn>;

<<<<<<< HEAD
export function codexExecSpy(): { args: string[][]; restore: () => void } {
  const previousImplementation = spawnMock.getMockImplementation() ?? actualChildProcess.spawn;
  const args: string[][] = [];
=======
export function codexExecSpy(): {
  args: string[][];
  envs: (Record<string, string> | undefined)[];
  restore: () => void;
} {
  const previousImplementation = spawnMock.getMockImplementation() ?? actualChildProcess.spawn;
  const args: string[][] = [];
  const envs: (Record<string, string> | undefined)[] = [];
>>>>>>> upstream/main

  spawnMock.mockImplementation(((...spawnArgs: Parameters<typeof child_process.spawn>) => {
    const commandArgs = spawnArgs[1];
    args.push(Array.isArray(commandArgs) ? [...commandArgs] : []);
<<<<<<< HEAD
=======
    const options = spawnArgs[2] as child_process.SpawnOptions | undefined;
    envs.push(options?.env as Record<string, string> | undefined);
>>>>>>> upstream/main
    return previousImplementation(...spawnArgs);
  }) as typeof actualChildProcess.spawn);

  return {
    args,
<<<<<<< HEAD
=======
    envs,
>>>>>>> upstream/main
    restore: () => {
      spawnMock.mockClear();
      spawnMock.mockImplementation(previousImplementation);
    },
  };
}
