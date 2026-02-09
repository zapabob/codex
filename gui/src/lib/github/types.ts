// SPDX-License-Identifier: Apache-2.0

export interface CommitAuthor {
  name: string;
  email: string;
  date: string;
}

export interface CommitData {
  sha: string;
  message: string;
  author: CommitAuthor;
  url: string;
}

export interface CommitFile {
  filename: string;
  status: "added" | "modified" | "deleted" | "renamed";
  additions: number;
  deletions: number;
  patch?: string;
}

export interface CommitStats {
  total: number;
  additions: number;
  deletions: number;
}

export interface CommitDetail {
  sha: string;
  message: string;
  author: string;
  date: string;
  files: CommitFile[];
  stats: CommitStats;
}

export interface PullRequestData {
  number: number;
  title: string;
  state: "open" | "closed";
  merged: boolean;
  mergeCommitSha?: string;
  createdAt: string;
  updatedAt: string;
  user: string;
  body?: string;
}

export interface PullRequestDetail {
  number: number;
  title: string;
  state: "open" | "closed";
  merged: boolean;
  mergeable: boolean;
  createdAt: string;
  updatedAt: string;
  closedAt?: string;
  mergedAt?: string;
  user: string;
  headBranch: string;
  baseBranch: string;
  commits: CommitData[];
}

export interface BranchData {
  name: string;
  commit: {
    sha: string;
    url: string;
  };
  protected: boolean;
}

export interface MergeEventData {
  sha: string;
  pullRequest: PullRequestData;
  mergedAt: string;
}

export interface RepositoryInfo {
  owner: string;
  repo: string;
  fullName: string;
  defaultBranch: string;
  description?: string;
  stars: number;
  forks: number;
}

export interface GitEvent {
  type: "commit" | "push" | "pull_request" | "merge" | "conflict";
  sha: string;
  repository: string;
  author: string;
  timestamp: string;
  data: Record<string, unknown>;
}
