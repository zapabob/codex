'use client'

import { useState, useEffect } from 'react'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/atoms/Button'
import { VirtualEnvironment, AIGeneration } from '@/app/virtual-os/page'
import {
  Sparkles,
  Send,
  Lightbulb,
  Code,
  RefreshCw,
  Copy,
  Download,
  ThumbsUp,
  ThumbsDown,
  MessageSquare,
  Settings,
  Wand2
} from 'lucide-react'

interface AIAssistantProps {
  selectedEnvironment: VirtualEnvironment | null
  onCodeGenerate: (generation: AIGeneration) => void
  generations: AIGeneration[]
}

export function AIAssistant({ selectedEnvironment, onCodeGenerate, generations }: AIAssistantProps) {
  const [prompt, setPrompt] = useState('')
  const [language, setLanguage] = useState('javascript')
  const [isGenerating, setIsGenerating] = useState(false)
  const [generatedCode, setGeneratedCode] = useState('')
  const [explanation, setExplanation] = useState('')
  const [suggestions, setSuggestions] = useState<string[]>([])
  const [confidence, setConfidence] = useState(0)

  const languages = [
    { value: 'javascript', label: 'JavaScript', icon: '🟨' },
    { value: 'typescript', label: 'TypeScript', icon: '🔷' },
    { value: 'python', label: 'Python', icon: '🐍' },
    { value: 'rust', label: 'Rust', icon: '🦀' },
    { value: 'go', label: 'Go', icon: '🐹' },
    { value: 'cpp', label: 'C++', icon: '🟦' },
    { value: 'java', label: 'Java', icon: '☕' },
  ]

  const promptTemplates = [
    {
      title: 'REST API Server',
      description: 'Create a simple REST API server',
      prompt: 'Create a REST API server with endpoints for CRUD operations on users. Include proper error handling and validation.',
    },
    {
      title: 'Data Processing Script',
      description: 'Process and analyze CSV data',
      prompt: 'Write a script to read a CSV file, process the data, and generate statistics and visualizations.',
    },
    {
      title: 'Web Scraper',
      description: 'Scrape data from websites',
      prompt: 'Create a web scraper that extracts product information from an e-commerce website and saves it to a database.',
    },
    {
      title: 'Unit Tests',
      description: 'Generate comprehensive unit tests',
      prompt: 'Write comprehensive unit tests for a user authentication system including login, registration, and password reset functionality.',
    },
    {
      title: 'Database Schema',
      description: 'Design database tables and relationships',
      prompt: 'Design a database schema for a blog platform with users, posts, comments, and categories. Include proper relationships and constraints.',
    },
    {
      title: 'Algorithm Implementation',
      description: 'Implement common algorithms',
      prompt: 'Implement various sorting algorithms (bubble sort, quick sort, merge sort) with performance comparisons.',
    },
  ]

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      alert('Please enter a prompt')
      return
    }

    if (!selectedEnvironment || selectedEnvironment.status !== 'running') {
      alert('Please select and start a virtual environment first')
      return
    }

    setIsGenerating(true)

    try {
      // Simulate AI code generation
      await new Promise(resolve => setTimeout(resolve, 2000 + Math.random() * 3000))

      // Generate mock code based on language and prompt
      const mockCode = generateMockCode(language, prompt)
      const mockExplanation = generateMockExplanation(language, prompt)
      const mockSuggestions = [
        'Add error handling for edge cases',
        'Consider adding logging for debugging',
        'Add input validation',
        'Consider performance optimizations',
      ]

      setGeneratedCode(mockCode)
      setExplanation(mockExplanation)
      setSuggestions(mockSuggestions)
      setConfidence(0.85 + Math.random() * 0.1) // 0.85-0.95

      const generation: AIGeneration = {
        id: `gen-${Date.now()}`,
        prompt: prompt,
        language: language,
        code: mockCode,
        explanation: mockExplanation,
        confidence: 0.85 + Math.random() * 0.1,
        timestamp: new Date(),
      }

      onCodeGenerate(generation)
    } catch (error) {
      console.error('Code generation failed:', error)
      alert('Code generation failed. Please try again.')
    } finally {
      setIsGenerating(false)
    }
  }

  const generateMockCode = (lang: string, userPrompt: string): string => {
    const basePrompt = userPrompt.toLowerCase()

    if (lang === 'javascript') {
      if (basePrompt.includes('api') || basePrompt.includes('server')) {
        return `const express = require('express');
const app = express();

app.use(express.json());

// User routes
app.get('/api/users', async (req, res) => {
    try {
        // Get all users from database
        const users = await User.find();
        res.json(users);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/api/users', async (req, res) => {
    try {
        const { name, email } = req.body;

        // Validate input
        if (!name || !email) {
            return res.status(400).json({ error: 'Name and email are required' });
        }

        const user = new User({ name, email });
        await user.save();
        res.status(201).json(user);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.get('/api/users/:id', async (req, res) => {
    try {
        const user = await User.findById(req.params.id);
        if (!user) {
            return res.status(404).json({ error: 'User not found' });
        }
        res.json(user);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.put('/api/users/:id', async (req, res) => {
    try {
        const { name, email } = req.body;
        const user = await User.findByIdAndUpdate(
            req.params.id,
            { name, email },
            { new: true, runValidators: true }
        );

        if (!user) {
            return res.status(404).json({ error: 'User not found' });
        }
        res.json(user);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.delete('/api/users/:id', async (req, res) => {
    try {
        const user = await User.findByIdAndDelete(req.params.id);
        if (!user) {
            return res.status(404).json({ error: 'User not found' });
        }
        res.json({ message: 'User deleted successfully' });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(\`Server running on port \${PORT}\`);
});`
      } else if (basePrompt.includes('data') || basePrompt.includes('csv')) {
        return `const fs = require('fs');
const csv = require('csv-parser');
const { createObjectCsvWriter } = require('csv-writer');

class DataProcessor {
    constructor() {
        this.data = [];
    }

    async loadCSV(filePath) {
        return new Promise((resolve, reject) => {
            const results = [];

            fs.createReadStream(filePath)
                .pipe(csv())
                .on('data', (data) => results.push(data))
                .on('end', () => {
                    this.data = results;
                    resolve(results);
                })
                .on('error', reject);
        });
    }

    generateStatistics() {
        if (this.data.length === 0) return null;

        const numericColumns = Object.keys(this.data[0]).filter(key => {
            return this.data.every(row => !isNaN(parseFloat(row[key])));
        });

        const stats = {};

        numericColumns.forEach(column => {
            const values = this.data.map(row => parseFloat(row[column])).filter(val => !isNaN(val));
            const sum = values.reduce((a, b) => a + b, 0);
            const mean = sum / values.length;
            const sorted = values.sort((a, b) => a - b);
            const median = sorted.length % 2 === 0
                ? (sorted[sorted.length / 2 - 1] + sorted[sorted.length / 2]) / 2
                : sorted[Math.floor(sorted.length / 2)];

            stats[column] = {
                count: values.length,
                sum: sum,
                mean: mean,
                median: median,
                min: Math.min(...values),
                max: Math.max(...values),
                stdDev: Math.sqrt(values.reduce((acc, val) => acc + Math.pow(val - mean, 2), 0) / values.length)
            };
        });

        return stats;
    }

    async saveProcessedData(outputPath, stats) {
        const csvWriter = createObjectCsvWriter({
            path: outputPath,
            header: [
                { id: 'metric', title: 'Metric' },
                { id: 'value', title: 'Value' }
            ]
        });

        const records = [];
        for (const [column, columnStats] of Object.entries(stats)) {
            records.push(
                { metric: \`\${column}_count\`, value: columnStats.count },
                { metric: \`\${column}_mean\`, value: columnStats.mean },
                { metric: \`\${column}_median\`, value: columnStats.median },
                { metric: \`\${column}_min\`, value: columnStats.min },
                { metric: \`\${column}_max\`, value: columnStats.max },
                { metric: \`\${column}_std_dev\`, value: columnStats.stdDev }
            );
        }

        await csvWriter.writeRecords(records);
    }

    createVisualization(stats) {
        // Generate simple text-based visualization
        let visualization = '';

        for (const [column, columnStats] of Object.entries(stats)) {
            visualization += \`\\n\${column} Distribution:\\n\`;
            visualization += \`Mean: \${columnStats.mean.toFixed(2)}\\n\`;
            visualization += \`Median: \${columnStats.median.toFixed(2)}\\n\`;
            visualization += \`Range: \${columnStats.min.toFixed(2)} - \${columnStats.max.toFixed(2)}\\n\`;

            // Simple histogram
            const range = columnStats.max - columnStats.min;
            const bins = 10;
            const binSize = range / bins;
            const histogram = new Array(bins).fill(0);

            // Simplified histogram generation
            visualization += \`Histogram (approx.): \${histogram.map(() => '█').join('')}\\n\`;
        }

        return visualization;
    }
}

// Usage example
async function main() {
    const processor = new DataProcessor();

    try {
        console.log('Loading CSV data...');
        await processor.loadCSV('data.csv');

        console.log('Generating statistics...');
        const stats = processor.generateStatistics();

        if (stats) {
            console.log('Saving processed data...');
            await processor.saveProcessedData('statistics.csv', stats);

            console.log('Creating visualization...');
            const visualization = processor.createVisualization(stats);
            console.log(visualization);
        }

        console.log('Data processing completed successfully!');
    } catch (error) {
        console.error('Error processing data:', error);
    }
}

if (require.main === module) {
    main();
}`
      }
    } else if (lang === 'python') {
      if (basePrompt.includes('data') || basePrompt.includes('csv')) {
        return `import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.preprocessing import StandardScaler
from sklearn.cluster import KMeans
import json

class DataAnalyzer:
    def __init__(self):
        self.data = None
        self.stats = None

    def load_data(self, file_path):
        """Load data from CSV file"""
        try:
            self.data = pd.read_csv(file_path)
            print(f"Loaded {len(self.data)} rows with {len(self.data.columns)} columns")
            return True
        except Exception as e:
            print(f"Error loading data: {e}")
            return False

    def clean_data(self):
        """Clean and preprocess the data"""
        if self.data is None:
            return False

        # Remove duplicates
        initial_rows = len(self.data)
        self.data = self.data.drop_duplicates()
        print(f"Removed {initial_rows - len(self.data)} duplicate rows")

        # Handle missing values
        missing_counts = self.data.isnull().sum()
        if missing_counts.sum() > 0:
            print("Missing values found:")
            for col, count in missing_counts[missing_counts > 0].items():
                print(f"  {col}: {count} missing values")

                # Fill numeric columns with median
                if self.data[col].dtype in ['int64', 'float64']:
                    median_val = self.data[col].median()
                    self.data[col] = self.data[col].fillna(median_val)
                # Fill categorical columns with mode
                else:
                    mode_val = self.data[col].mode().iloc[0] if not self.data[col].mode().empty else 'Unknown'
                    self.data[col] = self.data[col].fillna(mode_val)

        return True

    def generate_statistics(self):
        """Generate comprehensive statistics"""
        if self.data is None:
            return None

        self.stats = {}

        # Numeric columns statistics
        numeric_cols = self.data.select_dtypes(include=[np.number]).columns
        for col in numeric_cols:
            self.stats[col] = {
                'count': int(self.data[col].count()),
                'mean': float(self.data[col].mean()),
                'median': float(self.data[col].median()),
                'std': float(self.data[col].std()),
                'min': float(self.data[col].min()),
                'max': float(self.data[col].max()),
                'quartiles': {
                    '25%': float(self.data[col].quantile(0.25)),
                    '75%': float(self.data[col].quantile(0.75))
                }
            }

        # Categorical columns statistics
        categorical_cols = self.data.select_dtypes(include=['object']).columns
        for col in categorical_cols:
            value_counts = self.data[col].value_counts()
            self.stats[col] = {
                'unique_values': int(self.data[col].nunique()),
                'most_common': value_counts.index[0] if not value_counts.empty else None,
                'most_common_count': int(value_counts.iloc[0]) if not value_counts.empty else 0,
                'top_5': value_counts.head(5).to_dict()
            }

        return self.stats

    def create_visualizations(self, output_dir='visualizations'):
        """Create data visualizations"""
        if self.data is None:
            return False

        import os
        os.makedirs(output_dir, exist_ok=True)

        # Correlation heatmap for numeric columns
        numeric_data = self.data.select_dtypes(include=[np.number])
        if len(numeric_data.columns) > 1:
            plt.figure(figsize=(10, 8))
            correlation_matrix = numeric_data.corr()
            sns.heatmap(correlation_matrix, annot=True, cmap='coolwarm', center=0)
            plt.title('Correlation Heatmap')
            plt.tight_layout()
            plt.savefig(f'{output_dir}/correlation_heatmap.png', dpi=300, bbox_inches='tight')
            plt.close()

        # Distribution plots for numeric columns
        for col in numeric_data.columns[:5]:  # Limit to first 5 columns
            plt.figure(figsize=(10, 6))
            sns.histplot(self.data[col], kde=True)
            plt.title(f'Distribution of {col}')
            plt.xlabel(col)
            plt.ylabel('Frequency')
            plt.tight_layout()
            plt.savefig(f'{output_dir}/{col}_distribution.png', dpi=300, bbox_inches='tight')
            plt.close()

        # Bar plots for categorical columns
        categorical_data = self.data.select_dtypes(include=['object'])
        for col in categorical_data.columns[:3]:  # Limit to first 3 columns
            plt.figure(figsize=(12, 6))
            value_counts = self.data[col].value_counts().head(10)
            value_counts.plot(kind='bar')
            plt.title(f'Top 10 {col} Categories')
            plt.xlabel(col)
            plt.ylabel('Count')
            plt.xticks(rotation=45, ha='right')
            plt.tight_layout()
            plt.savefig(f'{output_dir}/{col}_categories.png', dpi=300, bbox_inches='tight')
            plt.close()

        print(f"Visualizations saved to {output_dir}/ directory")
        return True

    def perform_clustering(self, n_clusters=3):
        """Perform K-means clustering on numeric data"""
        if self.data is None:
            return None

        numeric_data = self.data.select_dtypes(include=[np.number])
        if len(numeric_data.columns) < 2:
            print("Not enough numeric columns for clustering")
            return None

        # Standardize the data
        scaler = StandardScaler()
        scaled_data = scaler.fit_transform(numeric_data)

        # Perform clustering
        kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
        clusters = kmeans.fit_predict(scaled_data)

        # Add cluster labels to data
        self.data['cluster'] = clusters

        cluster_stats = {}
        for i in range(n_clusters):
            cluster_data = self.data[self.data['cluster'] == i]
            cluster_stats[f'cluster_{i}'] = {
                'size': int(len(cluster_data)),
                'percentage': float(len(cluster_data) / len(self.data) * 100),
                'centroid': kmeans.cluster_centers_[i].tolist()
            }

        return {
            'cluster_labels': clusters.tolist(),
            'cluster_stats': cluster_stats,
            'inertia': float(kmeans.inertia_),
            'n_clusters': n_clusters
        }

    def export_results(self, output_file='analysis_results.json'):
        """Export analysis results to JSON"""
        if self.stats is None:
            return False

        results = {
            'dataset_info': {
                'rows': int(len(self.data)) if self.data is not None else 0,
                'columns': int(len(self.data.columns)) if self.data is not None else 0,
                'column_types': self.data.dtypes.astype(str).to_dict() if self.data is not None else {}
            },
            'statistics': self.stats,
            'timestamp': pd.Timestamp.now().isoformat()
        }

        try:
            with open(output_file, 'w') as f:
                json.dump(results, f, indent=2, default=str)
            print(f"Results exported to {output_file}")
            return True
        except Exception as e:
            print(f"Error exporting results: {e}")
            return False

def main():
    analyzer = DataAnalyzer()

    # Load and process data
    if analyzer.load_data('data.csv'):
        analyzer.clean_data()
        stats = analyzer.generate_statistics()

        if stats:
            analyzer.create_visualizations()
            clustering_results = analyzer.perform_clustering()
            analyzer.export_results()

            print("\\nAnalysis completed successfully!")
            print(f"Processed {len(analyzer.data)} rows of data")

if __name__ == "__main__":
    main()`
      }
    }

    // Default fallback
    return `// Generated code for: ${userPrompt}
// Language: ${lang}
// Confidence: High

console.log("AI-generated code for ${userPrompt}");
console.log("This is a placeholder implementation.");
console.log("Please refine the prompt for more specific code generation.");
`;
  };
}
