'use client';

import { useEffect, useCallback, useRef } from 'react';

export type KeyboardShortcut = {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
  description: string;
  action: () => void;
};

interface UseHotkeysOptions {
  /**
   * Enable or disable the hotkeys
   */
  enabled?: boolean;
  
  /**
   * Prevent shortcuts when user is typing in input fields
   */
  ignoreInputFields?: boolean;
}

/**
 * Hook to register keyboard shortcuts
 * @param shortcuts - Array of keyboard shortcuts to register
 * @param options - Configuration options
 */
export function useHotkeys(
  shortcuts: KeyboardShortcut[],
  options: UseHotkeysOptions = {}
) {
  const { enabled = true, ignoreInputFields = true } = options;
  const shortcutsRef = useRef(shortcuts);

  // Update shortcuts ref when they change
  useEffect(() => {
    shortcutsRef.current = shortcuts;
  }, [shortcuts]);

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!enabled) return;

      // Check if we should ignore this event (e.g., when typing in input)
      if (ignoreInputFields) {
        const target = event.target as HTMLElement;
        const tagName = target.tagName.toLowerCase();
        const isEditable =
          tagName === 'input' ||
          tagName === 'textarea' ||
          target.isContentEditable;

        if (isEditable) {
          return;
        }
      }

      // Find matching shortcut
      for (const shortcut of shortcutsRef.current) {
        const keyMatches =
          event.key.toLowerCase() === shortcut.key.toLowerCase();
        const ctrlMatches = !shortcut.ctrl || event.ctrlKey;
        const shiftMatches = !shortcut.shift || event.shiftKey;
        const altMatches = !shortcut.alt || event.altKey;
        const metaMatches = !shortcut.meta || event.metaKey;

        // Check if Ctrl/Cmd is pressed when required
        const modifierMatches =
          (shortcut.ctrl && event.ctrlKey) ||
          (shortcut.meta && event.metaKey) ||
          (!shortcut.ctrl && !shortcut.meta);

        if (
          keyMatches &&
          ctrlMatches &&
          shiftMatches &&
          altMatches &&
          metaMatches &&
          modifierMatches
        ) {
          event.preventDefault();
          shortcut.action();
          break;
        }
      }
    },
    [enabled, ignoreInputFields]
  );

  useEffect(() => {
    if (enabled) {
      window.addEventListener('keydown', handleKeyDown);
      return () => {
        window.removeEventListener('keydown', handleKeyDown);
      };
    }
  }, [enabled, handleKeyDown]);
}

/**
 * Get the keyboard shortcut display text
 * @param shortcut - The keyboard shortcut
 * @returns A formatted string for display (e.g., "⌘+Enter" or "Ctrl+Enter")
 */
export function getShortcutDisplay(shortcut: KeyboardShortcut): string {
  const isMac = typeof navigator !== 'undefined' && navigator.platform.toUpperCase().indexOf('MAC') >= 0;
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

  // Capitalize first letter of key
  const key = shortcut.key.charAt(0).toUpperCase() + shortcut.key.slice(1);
  parts.push(key);

  return parts.join('+');
}

/**
 * Get the ARIA keyboard shortcut format
 * @param shortcut - The keyboard shortcut
 * @returns A formatted string for aria-keyshortcuts attribute
 */
export function getAriaKeyshortcuts(shortcut: KeyboardShortcut): string {
  const parts: string[] = [];

  if (shortcut.meta) {
    parts.push('Meta');
  }
  if (shortcut.ctrl) {
    parts.push('Control');
  }
  if (shortcut.shift) {
    parts.push('Shift');
  }
  if (shortcut.alt) {
    parts.push('Alt');
  }
  parts.push(shortcut.key);

  return parts.join('+');
}
