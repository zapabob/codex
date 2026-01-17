#!/usr/bin/env node

/**
 * Codex Extended v2.11.0 Feedback Collection & Analysis Script
 *
 * This script helps collect and analyze community feedback from various sources:
 * - GitHub Issues/PRs with feedback label
 * - Discord messages (if API access available)
 * - Direct user surveys
 * - Social media mentions
 */

const fs = require('fs');
const path = require('path');
const https = require('https');

class FeedbackCollector {
    constructor() {
        this.feedbackDir = path.join(__dirname, '..', 'feedback');
        this.ensureDirectories();
    }

    ensureDirectories() {
        if (!fs.existsSync(this.feedbackDir)) {
            fs.mkdirSync(this.feedbackDir, { recursive: true });
        }
    }

    async collectGitHubFeedback() {
        console.log('🔍 Collecting GitHub feedback...');

        try {
            // GitHub API doesn't require authentication for public repos
            const issues = await this.fetchGitHubIssues();

            const feedbackIssues = issues.filter(issue =>
                issue.labels.some(label =>
                    label.name.toLowerCase().includes('feedback') ||
                    issue.title.toLowerCase().includes('feedback') ||
                    issue.body.toLowerCase().includes('feedback')
                )
            );

            console.log(`📊 Found ${feedbackIssues.length} feedback-related issues`);

            this.saveFeedback('github-issues.json', {
                timestamp: new Date().toISOString(),
                total_issues: feedbackIssues.length,
                issues: feedbackIssues.map(issue => ({
                    number: issue.number,
                    title: issue.title,
                    state: issue.state,
                    created_at: issue.created_at,
                    updated_at: issue.updated_at,
                    labels: issue.labels.map(l => l.name),
                    author: issue.user.login,
                    body_preview: issue.body.substring(0, 500) + '...',
                    url: issue.html_url,
                    comments_count: issue.comments
                }))
            });

            return feedbackIssues;

        } catch (error) {
            console.error('❌ Failed to collect GitHub feedback:', error.message);
            return [];
        }
    }

    async fetchGitHubIssues() {
        return new Promise((resolve, reject) => {
            const options = {
                hostname: 'api.github.com',
                path: '/repos/zapabob/Codex/issues?state=all&per_page=100',
                method: 'GET',
                headers: {
                    'User-Agent': 'Codex-Feedback-Collector/1.0',
                    'Accept': 'application/vnd.github.v3+json'
                }
            };

            const req = https.request(options, (res) => {
                let data = '';

                res.on('data', (chunk) => {
                    data += chunk;
                });

                res.on('end', () => {
                    try {
                        const issues = JSON.parse(data);
                        resolve(issues);
                    } catch (error) {
                        reject(error);
                    }
                });
            });

            req.on('error', (error) => {
                reject(error);
            });

            req.setTimeout(10000, () => {
                req.destroy();
                reject(new Error('Request timeout'));
            });

            req.end();
        });
    }

    async analyzeFeedback() {
        console.log('📈 Analyzing feedback data...');

        try {
            const githubData = this.loadFeedback('github-issues.json');

            if (!githubData) {
                console.log('ℹ️ No feedback data found. Run collection first.');
                return;
            }

            const analysis = {
                timestamp: new Date().toISOString(),
                summary: {
                    total_feedback_items: githubData.total_issues,
                    analysis_period: 'All time',
                    last_updated: githubData.timestamp
                },
                categories: this.categorizeFeedback(githubData.issues),
                sentiment: this.analyzeSentiment(githubData.issues),
                feature_requests: this.extractFeatureRequests(githubData.issues),
                top_issues: this.identifyTopIssues(githubData.issues),
                user_satisfaction: this.calculateUserSatisfaction(githubData.issues),
                recommendations: this.generateRecommendations(githubData.issues)
            };

            this.saveFeedback('analysis-report.json', analysis);
            this.generateMarkdownReport(analysis);

            console.log('✅ Analysis complete! Check feedback/analysis-report.md');

        } catch (error) {
            console.error('❌ Analysis failed:', error.message);
        }
    }

    categorizeFeedback(issues) {
        const categories = {
            'feature-request': 0,
            'bug-report': 0,
            'performance': 0,
            'usability': 0,
            'documentation': 0,
            'security': 0,
            'other': 0
        };

        issues.forEach(issue => {
            const body = (issue.title + ' ' + issue.body_preview).toLowerCase();

            if (body.includes('feature') || body.includes('add') || body.includes('implement')) {
                categories['feature-request']++;
            } else if (body.includes('bug') || body.includes('error') || body.includes('fix')) {
                categories['bug-report']++;
            } else if (body.includes('performance') || body.includes('slow') || body.includes('speed')) {
                categories['performance']++;
            } else if (body.includes('usability') || body.includes('ui') || body.includes('ux')) {
                categories['usability']++;
            } else if (body.includes('documentation') || body.includes('docs') || body.includes('guide')) {
                categories['documentation']++;
            } else if (body.includes('security') || body.includes('privacy') || body.includes('safe')) {
                categories['security']++;
            } else {
                categories['other']++;
            }
        });

        return categories;
    }

    analyzeSentiment(issues) {
        let positive = 0;
        let neutral = 0;
        let negative = 0;

        const positiveWords = ['great', 'awesome', 'excellent', 'fantastic', 'love', 'amazing', 'perfect', 'good', 'helpful', 'useful', 'easy'];
        const negativeWords = ['bad', 'terrible', 'awful', 'horrible', 'hate', 'worst', 'broken', 'difficult', 'confusing', 'slow', 'buggy'];

        issues.forEach(issue => {
            const text = (issue.title + ' ' + issue.body_preview).toLowerCase();

            const positiveCount = positiveWords.filter(word => text.includes(word)).length;
            const negativeCount = negativeWords.filter(word => text.includes(word)).length;

            if (positiveCount > negativeCount) {
                positive++;
            } else if (negativeCount > positiveCount) {
                negative++;
            } else {
                neutral++;
            }
        });

        return {
            positive,
            neutral,
            negative,
            total: issues.length,
            positive_percentage: Math.round((positive / issues.length) * 100)
        };
    }

    extractFeatureRequests(issues) {
        return issues
            .filter(issue => issue.title.toLowerCase().includes('feature') ||
                            issue.body_preview.toLowerCase().includes('would like') ||
                            issue.body_preview.toLowerCase().includes('add'))
            .map(issue => ({
                title: issue.title,
                author: issue.author,
                created: issue.created_at,
                url: issue.url
            }))
            .slice(0, 10); // Top 10
    }

    identifyTopIssues(issues) {
        const issueCounts = {};

        issues.forEach(issue => {
            const text = (issue.title + ' ' + issue.body_preview).toLowerCase();
            const keywords = this.extractKeywords(text);

            keywords.forEach(keyword => {
                issueCounts[keyword] = (issueCounts[keyword] || 0) + 1;
            });
        });

        return Object.entries(issueCounts)
            .sort(([,a], [,b]) => b - a)
            .slice(0, 10)
            .map(([keyword, count]) => ({ keyword, count }));
    }

    extractKeywords(text) {
        const keywords = [];
        const commonWords = ['the', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by', 'an', 'a', 'is', 'are', 'was', 'were', 'be', 'been', 'being', 'have', 'has', 'had', 'do', 'does', 'did', 'will', 'would', 'could', 'should', 'may', 'might', 'must', 'can', 'this', 'that', 'these', 'those', 'i', 'you', 'he', 'she', 'it', 'we', 'they', 'me', 'him', 'her', 'us', 'them'];

        const words = text.toLowerCase().match(/\b\w{4,}\b/g) || [];

        words.forEach(word => {
            if (!commonWords.includes(word)) {
                keywords.push(word);
            }
        });

        return [...new Set(keywords)]; // Remove duplicates
    }

    calculateUserSatisfaction(issues) {
        // This would be more accurate with structured feedback data
        // For now, using simple heuristics
        const satisfiedIndicators = ['great', 'awesome', 'excellent', 'fantastic', 'love', 'amazing', 'perfect', 'good', 'helpful', 'useful', 'easy', 'satisfied'];
        const dissatisfiedIndicators = ['bad', 'terrible', 'awful', 'horrible', 'hate', 'worst', 'broken', 'difficult', 'confusing', 'slow', 'buggy', 'dissatisfied'];

        let satisfied = 0;
        let dissatisfied = 0;

        issues.forEach(issue => {
            const text = (issue.title + ' ' + issue.body_preview).toLowerCase();

            const satisfiedCount = satisfiedIndicators.filter(word => text.includes(word)).length;
            const dissatisfiedCount = dissatisfiedIndicators.filter(word => text.includes(word)).length;

            if (satisfiedCount > dissatisfiedCount) {
                satisfied++;
            } else if (dissatisfiedCount > satisfiedCount) {
                dissatisfied++;
            }
        });

        const total = satisfied + dissatisfied;
        const satisfactionRate = total > 0 ? Math.round((satisfied / total) * 100) : 0;

        return {
            satisfied,
            dissatisfied,
            neutral: issues.length - total,
            satisfaction_rate: satisfactionRate
        };
    }

    generateRecommendations(issues) {
        const recommendations = [];

        // Analyze categories and generate recommendations
        const categories = this.categorizeFeedback(issues);

        if (categories['feature-request'] > categories['bug-report']) {
            recommendations.push({
                priority: 'high',
                type: 'feature-development',
                description: 'Focus on implementing requested features based on user feedback'
            });
        }

        if (categories['performance'] > 5) {
            recommendations.push({
                priority: 'high',
                type: 'performance-optimization',
                description: 'Address performance issues reported by users'
            });
        }

        if (categories['documentation'] > 3) {
            recommendations.push({
                priority: 'medium',
                type: 'documentation-improvement',
                description: 'Improve documentation based on user feedback'
            });
        }

        recommendations.push({
            priority: 'medium',
            type: 'user-engagement',
            description: 'Increase community engagement through regular updates and surveys'
        });

        return recommendations;
    }

    saveFeedback(filename, data) {
        const filePath = path.join(this.feedbackDir, filename);
        fs.writeFileSync(filePath, JSON.stringify(data, null, 2));
        console.log(`💾 Saved ${filename}`);
    }

    loadFeedback(filename) {
        const filePath = path.join(this.feedbackDir, filename);
        if (fs.existsSync(filePath)) {
            return JSON.parse(fs.readFileSync(filePath, 'utf8'));
        }
        return null;
    }

    generateMarkdownReport(analysis) {
        const report = `# 📊 Codex Extended v2.11.0 Feedback Analysis Report

Generated on: ${new Date().toISOString()}

## 📈 Summary

- **Total Feedback Items**: ${analysis.summary.total_feedback_items}
- **Analysis Period**: ${analysis.summary.analysis_period}
- **Last Updated**: ${analysis.summary.last_updated}

## 🎯 Categories

| Category | Count |
|----------|-------|
${Object.entries(analysis.categories).map(([cat, count]) =>
    `| ${cat.replace('-', ' ').replace(/\b\w/g, l => l.toUpperCase())} | ${count} |`
).join('\n')}

## 😊 Sentiment Analysis

- **Positive**: ${analysis.sentiment.positive} (${analysis.sentiment.positive_percentage}%)
- **Neutral**: ${analysis.sentiment.neutral}
- **Negative**: ${analysis.sentiment.negative}
- **Total Analyzed**: ${analysis.sentiment.total}

## 🏆 User Satisfaction

- **Satisfaction Rate**: ${analysis.user_satisfaction.satisfaction_rate}%
- **Satisfied Users**: ${analysis.user_satisfaction.satisfied}
- **Dissatisfied Users**: ${analysis.user_satisfaction.dissatisfied}
- **Neutral**: ${analysis.user_satisfaction.neutral}

## 🚀 Top Feature Requests

${analysis.feature_requests.map((req, i) =>
    `${i + 1}. **[${req.title}](${req.url})**\n   - By: ${req.author}\n   - Created: ${new Date(req.created).toLocaleDateString()}`
).join('\n\n')}

## ⚠️ Top Issues

${analysis.top_issues.map((issue, i) =>
    `${i + 1}. **${issue.keyword}** (${issue.count} mentions)`
).join('\n')}

## 💡 Recommendations

${analysis.recommendations.map((rec, i) =>
    `### ${i + 1}. ${rec.description}\n**Priority**: ${rec.priority.toUpperCase()}\n**Type**: ${rec.type.replace('-', ' ')}\n`
).join('\n')}

## 📋 Next Steps

1. **Address High Priority Issues** - Focus on performance and critical bugs
2. **Implement Popular Features** - Review top feature requests
3. **Improve Documentation** - Address documentation feedback
4. **Community Engagement** - Regular updates and surveys
5. **User Support** - Better help channels and response times

---

*This report was generated automatically by the Codex Extended feedback analysis system.*
`;

        const reportPath = path.join(this.feedbackDir, 'analysis-report.md');
        fs.writeFileSync(reportPath, report);
        console.log('📄 Generated markdown report');
    }

    async runFullAnalysis() {
        console.log('🚀 Starting Codex Extended Feedback Collection & Analysis');
        console.log('=' .repeat(60));

        await this.collectGitHubFeedback();
        await this.analyzeFeedback();

        console.log('=' .repeat(60));
        console.log('✅ Feedback analysis complete!');
        console.log('📁 Check the feedback/ directory for results');
    }
}

// CLI interface
const args = process.argv.slice(2);
const collector = new FeedbackCollector();

if (args.includes('--collect')) {
    collector.collectGitHubFeedback();
} else if (args.includes('--analyze')) {
    collector.analyzeFeedback();
} else if (args.includes('--full')) {
    collector.runFullAnalysis();
} else {
    console.log(`
Codex Extended Feedback Collector v1.0

Usage:
  node collect-feedback.js --collect    # Collect feedback from GitHub
  node collect-feedback.js --analyze    # Analyze existing feedback data
  node collect-feedback.js --full       # Run full collection and analysis

Output:
  - feedback/github-issues.json         # Raw GitHub feedback data
  - feedback/analysis-report.json       # Structured analysis data
  - feedback/analysis-report.md         # Human-readable report

Examples:
  # Collect feedback from GitHub
  node collect-feedback.js --collect

  # Analyze existing data
  node collect-feedback.js --analyze

  # Full pipeline
  node collect-feedback.js --full
`);
}