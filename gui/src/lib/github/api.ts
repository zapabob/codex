// SPDX-License-Identifier: Apache-2.0

import { Octokit } from "@octokit/rest";
import { loadConfig, getGitHubToken } from "@/lib/config";
import type {
  CommitData,
  CommitDetail,
  PullRequestData,
  PullRequestDetail,
  BranchData,
  RepositoryInfo,
} from "./types";

export interface GitHubAPIConfig {
  owner: string;
  repo: string;
  token?: string;
}

export class GitHubAPI {
  private octokit: Octokit;
  private owner: string;
  private repo: string;

  constructor(config?: GitHubAPIConfig) {
    const fullConfig = loadConfig();

    this.owner = config?.owner || fullConfig.github.owner;
    this.repo = config?.repo || fullConfig.github.repo;
    const token = config?.token || getGitHubToken();

    this.octokit = new Octokit({
      auth: token,
      baseUrl: "https://api.github.com",
      userAgent: "Codex-GUI/1.0",
    });
  }

  async getRepositoryInfo(): Promise<RepositoryInfo> {
    const { data } = await this.octokit.repos.get({
      owner: this.owner,
      repo: this.repo,
    });

    return {
      owner: data.owner?.login || this.owner,
      repo: data.name,
      fullName: data.full_name,
      defaultBranch: data.default_branch || "main",
      description: data.description,
      stars: data.stargazers_count,
      forks: data.forks_count,
    };
  }

  async fetchCommits(options?: {
    sha?: string;
    path?: string;
    since?: string;
    until?: string;
    perPage?: number;
  }): Promise<CommitData[]> {
    const { data } = await this.octokit.repos.listCommits({
      owner: this.owner,
      repo: this.repo,
      sha: options?.sha,
      path: options?.path,
      since: options?.since,
      until: options?.until,
      per_page: options?.perPage || 100,
    });

    return data.map((commit) => ({
      sha: commit.sha,
      message: commit.commit.message,
      author: {
        name: commit.commit.author?.name || "Unknown",
        email: commit.commit.author?.email || "",
        date: commit.commit.author?.date || "",
      },
      url: commit.html_url || "",
    }));
  }

  async fetchCommitDetail(sha: string): Promise<CommitDetail> {
    const { data } = await this.octokit.repos.getCommit({
      owner: this.owner,
      repo: this.repo,
      commit_sha: sha,
    });

    return {
      sha: data.sha,
      message: data.commit.message,
      author: data.commit.author?.name || "Unknown",
      date: data.commit.author?.date || "",
      files: (data.files || []).map((file) => ({
        filename: file.filename,
        status: file.status as "added" | "modified" | "deleted" | "renamed",
        additions: file.additions,
        deletions: file.deletions,
        patch: file.patch,
      })),
      stats: {
        total: data.stats?.total || 0,
        additions: data.stats?.additions || 0,
        deletions: data.stats?.deletions || 0,
      },
    };
  }

  async fetchPullRequests(
    state: "open" | "closed" | "all" = "all",
  ): Promise<PullRequestData[]> {
    const { data } = await this.octokit.pulls.list({
      owner: this.owner,
      repo: this.repo,
      state,
      per_page: 100,
    });

    return data.map((pr) => ({
      number: pr.number,
      title: pr.title,
      state: pr.state as "open" | "closed",
      merged: pr.merged_at !== null,
      mergeCommitSha: pr.merge_commit_sha || undefined,
      createdAt: pr.created_at,
      updatedAt: pr.updated_at,
      user: pr.user?.login || "Unknown",
      body: pr.body || undefined,
    }));
  }

  async fetchPRDetail(prNumber: number): Promise<PullRequestDetail> {
    const { data: prData } = await this.octokit.pulls.get({
      owner: this.owner,
      repo: this.repo,
      pull_number: prNumber,
    });

    const { data: commitsData } = await this.octokit.pulls.listCommits({
      owner: this.owner,
      repo: this.repo,
      pull_number: prNumber,
    });

    const commits: CommitData[] = commitsData.map((commit) => ({
      sha: commit.sha,
      message: commit.commit.message,
      author: {
        name: commit.commit.author?.name || "Unknown",
        email: commit.commit.author?.email || "",
        date: commit.commit.author?.date || "",
      },
      url: commit.html_url || "",
    }));

    return {
      number: prData.number,
      title: prData.title,
      state: prData.state as "open" | "closed",
      merged: prData.merged_at !== null,
      mergeable: prData.mergeable || false,
      createdAt: prData.created_at,
      updatedAt: prData.updated_at,
      closedAt: prData.closed_at || undefined,
      mergedAt: prData.merged_at || undefined,
      user: prData.user?.login || "Unknown",
      headBranch: prData.head.ref,
      baseBranch: prData.base.ref,
      commits,
    };
  }

  async fetchPRsForCommit(sha: string): Promise<PullRequestData[]> {
    const { data } =
      await this.octokit.repos.listPullRequestsAssociatedWithCommit({
        owner: this.owner,
        repo: this.repo,
        commit_sha: sha,
      });

    return data.map((pr) => ({
      number: pr.number,
      title: pr.title,
      state: pr.state as "open" | "closed",
      merged: pr.merged_at !== null,
      mergeCommitSha: pr.merge_commit_sha || undefined,
      createdAt: pr.created_at,
      updatedAt: pr.updated_at,
      user: pr.user?.login || "Unknown",
    }));
  }

  async fetchBranches(): Promise<BranchData[]> {
    const { data } = await this.octokit.repos.listBranches({
      owner: this.owner,
      repo: this.repo,
      per_page: 100,
    });

    return data.map((branch) => ({
      name: branch.name,
      commit: {
        sha: branch.commit.sha,
        url: branch.commit.url,
      },
      protected: branch.protected,
    }));
  }

  async checkMergeStatus(
    prNumber: number,
  ): Promise<{ merged: boolean; mergeCommitSha?: string }> {
    const { data } = await this.octokit.pulls.checkIfMerged({
      owner: this.owner,
      repo: this.repo,
      pull_number: prNumber,
    });

    return {
      merged: true,
      mergeCommitSha: data.sha,
    };
  }

  async searchCommits(query: string): Promise<CommitData[]> {
    const { data } = await this.octokit.search.commits({
      q: `${query} repo:${this.owner}/${this.repo}`,
      per_page: 50,
    });

    return data.items.map((item) => ({
      sha: item.sha,
      message: item.commit.message,
      author: {
        name: item.commit.author?.name || "Unknown",
        email: item.commit.author?.email || "",
        date: item.commit.author?.date || "",
      },
      url: item.html_url || "",
    }));
  }

  static create(): GitHubAPI {
    return new GitHubAPI();
  }
}

export function useGitHubAPI(owner?: string, repo?: string): GitHubAPI {
  return new GitHubAPI({ owner, repo });
}
