'use client';

import React, { createContext, useContext, ReactNode, useState, useCallback } from 'react';
import { useHotkeys, KeyboardShortcut } from '@/hooks/useHotkeys';
import { useCodex } from './CodexContext';

interface KeyboardShortcutsContextType {
  shortcuts: KeyboardShortcut[];
  showHelp: boolean;
  toggleHelp: () => void;
}

const KeyboardShortcutsContext = createContext<KeyboardShortcutsContextType | undefined>(undefined);

export function KeyboardShortcutsProvider({ children }: { children: ReactNode }) {
  const { executeCommand } = useCodex();
  const [showHelp, setShowHelp] = useState(false);

  const toggleHelp = useCallback(() => {
    setShowHelp((prev) => !prev);
  }, []);

  // Define global keyboard shortcuts
  const shortcuts: KeyboardShortcut[] = [
    {
      key: 'Enter',
      meta: true,
      description: 'Run the current command or code',
      action: () => {
        console.log('Run command (⌘+Enter)');
        // This will be handled by the specific page/component
      },
    },
    {
      key: 'Enter',
      ctrl: true,
      description: 'Run the current command or code',
      action: () => {
        console.log('Run command (Ctrl+Enter)');
        // This will be handled by the specific page/component
      },
    },
    {
      key: 's',
      meta: true,
      description: 'Commit changes to Git',
      action: () => {
        console.log('Commit (⌘+S)');
        executeCommand('git add . && git commit', process.cwd());
      },
    },
    {
      key: 's',
      ctrl: true,
      description: 'Commit changes to Git',
      action: () => {
        console.log('Commit (Ctrl+S)');
        executeCommand('git add . && git commit', process.cwd());
      },
    },
    {
      key: 's',
      meta: true,
      shift: true,
      description: 'Push changes to remote',
      action: () => {
        console.log('Push (⌘+Shift+S)');
        executeCommand('git push', process.cwd());
      },
    },
    {
      key: 's',
      ctrl: true,
      shift: true,
      description: 'Push changes to remote',
      action: () => {
        console.log('Push (Ctrl+Shift+S)');
        executeCommand('git push', process.cwd());
      },
    },
    {
      key: 'd',
      meta: true,
      description: 'Show git diff',
      action: () => {
        console.log('Diff (⌘+D)');
        executeCommand('git diff', process.cwd());
      },
    },
    {
      key: 'd',
      ctrl: true,
      description: 'Show git diff',
      action: () => {
        console.log('Diff (Ctrl+D)');
        executeCommand('git diff', process.cwd());
      },
    },
    {
      key: 'z',
      meta: true,
      description: 'Revert last change',
      action: () => {
        console.log('Revert (⌘+Z)');
        executeCommand('git checkout HEAD~1', process.cwd());
      },
    },
    {
      key: 'z',
      ctrl: true,
      description: 'Revert last change',
      action: () => {
        console.log('Revert (Ctrl+Z)');
        executeCommand('git checkout HEAD~1', process.cwd());
      },
    },
    {
      key: '?',
      shift: true,
      description: 'Show keyboard shortcuts help',
      action: () => {
        toggleHelp();
      },
    },
  ];

  // Register global hotkeys
  useHotkeys(shortcuts, { enabled: true, ignoreInputFields: true });

  const value: KeyboardShortcutsContextType = {
    shortcuts,
    showHelp,
    toggleHelp,
  };

  return (
    <KeyboardShortcutsContext.Provider value={value}>
      {children}
      {showHelp && <KeyboardShortcutsHelp onClose={toggleHelp} shortcuts={shortcuts} />}
    </KeyboardShortcutsContext.Provider>
  );
}

export function useKeyboardShortcuts() {
  const context = useContext(KeyboardShortcutsContext);
  if (context === undefined) {
    throw new Error('useKeyboardShortcuts must be used within a KeyboardShortcutsProvider');
  }
  return context;
}

// Keyboard shortcuts help modal
function KeyboardShortcutsHelp({
  onClose,
  shortcuts,
}: {
  onClose: () => void;
  shortcuts: KeyboardShortcut[];
}) {
  const isMac = typeof navigator !== 'undefined' && navigator.platform.toUpperCase().indexOf('MAC') >= 0;

  const formatShortcut = (shortcut: KeyboardShortcut): string => {
    const parts: string[] = [];
    
    if (shortcut.ctrl) {
      parts.push(isMac ? '⌃' : 'Ctrl');
    }
    if (shortcut.shift) {
      parts.push(isMac ? '⇧' : 'Shift');
    }
    if (shortcut.alt) {
      parts.push(isMac ? '⌥' : 'Alt');
    }
    if (shortcut.meta) {
      parts.push(isMac ? '⌘' : 'Win');
    }
    
    const key = shortcut.key === '?' ? '?' : shortcut.key.charAt(0).toUpperCase() + shortcut.key.slice(1);
    parts.push(key);
    
    return parts.join('+');
  };

  // Group shortcuts by category
  const grouped = shortcuts.reduce((acc, shortcut) => {
    const display = formatShortcut(shortcut);
    if (!acc.find(s => s.display === display)) {
      acc.push({ ...shortcut, display });
    }
    return acc;
  }, [] as (KeyboardShortcut & { display: string })[]);

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full m-4 shadow-xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
            Keyboard Shortcuts
          </h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            aria-label="Close"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <div className="space-y-2">
          {grouped.map((shortcut, index) => (
            <div
              key={index}
              className="flex items-center justify-between py-2 px-3 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
            >
              <span className="text-gray-700 dark:text-gray-300">
                {shortcut.description}
              </span>
              <kbd className="px-3 py-1 text-sm font-mono bg-gray-200 dark:bg-gray-600 text-gray-800 dark:text-gray-200 rounded border border-gray-300 dark:border-gray-500">
                {shortcut.display}
              </kbd>
            </div>
          ))}
        </div>

        <div className="mt-6 text-sm text-gray-500 dark:text-gray-400">
          Press <kbd className="px-2 py-1 bg-gray-200 dark:bg-gray-600 rounded">?</kbd> to
          toggle this help
        </div>
      </div>
    </div>
  );
}
