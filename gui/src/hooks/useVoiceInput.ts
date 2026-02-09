import { useState, useCallback, useRef, useEffect } from "react";

interface UseVoiceInputOptions {
  continuous?: boolean;
  interimResults?: boolean;
  onResult?: (transcript: string, isFinal: boolean) => void;
  onError?: (error: string) => void;
  onStart?: () => void;
  onEnd?: () => void;
}

interface UseVoiceInputReturn {
  isListening: boolean;
  transcript: string;
  isSupported: boolean;
  startListening: () => void;
  stopListening: () => void;
  clearTranscript: () => void;
  error: string | null;
}

export function useVoiceInput(
  options: UseVoiceInputOptions = {},
): UseVoiceInputReturn {
  const {
    continuous = true,
    interimResults = true,
    onResult,
    onError,
    onStart,
    onEnd,
  } = options;

  const [isListening, setIsListening] = useState(false);
  const [transcript, setTranscript] = useState("");
  const [error, setError] = useState<string | null>(null);

  const recognitionRef = useRef<SpeechRecognition | null>(null);
  const isSupported =
    typeof window !== "undefined" &&
    ("SpeechRecognition" in window || "webkitSpeechRecognition" in window);

  useEffect(() => {
    if (!isSupported) return;

    const SpeechRecognition =
      window.SpeechRecognition || window.webkitSpeechRecognition;
    recognitionRef.current = new SpeechRecognition();
    recognitionRef.current.continuous = continuous;
    recognitionRef.current.interimResults = interimResults;
    recognitionRef.current.lang = "en-US";

    recognitionRef.current.onresult = (event) => {
      let interimTranscript = "";
      let finalTranscript = "";

      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i];
        if (result.isFinal) {
          finalTranscript += result[0].transcript;
        } else {
          interimTranscript += result[0].transcript;
        }
      }

      if (finalTranscript) {
        setTranscript((prev) => prev + finalTranscript);
        onResult?.(finalTranscript, true);
      }

      if (interimTranscript) {
        onResult?.(interimTranscript, false);
      }
    };

    recognitionRef.current.onerror = (event) => {
      const errorMessage =
        event.error === "no-speech"
          ? "No speech detected. Please try again."
          : `Speech recognition error: ${event.error}`;
      setError(errorMessage);
      onError?.(errorMessage);
      setIsListening(false);
    };

    recognitionRef.current.onend = () => {
      setIsListening(false);
      onEnd?.();
    };

    return () => {
      recognitionRef.current?.abort();
    };
  }, [isSupported, continuous, interimResults, onResult, onError, onEnd]);

  const startListening = useCallback(() => {
    if (!recognitionRef.current || isListening) return;

    setError(null);
    recognitionRef.current.start();
    setIsListening(true);
    onStart?.();
  }, [isListening, onStart]);

  const stopListening = useCallback(() => {
    if (!recognitionRef.current || !isListening) return;

    recognitionRef.current.stop();
    setIsListening(false);
    onEnd?.();
  }, [isListening, onEnd]);

  const clearTranscript = useCallback(() => {
    setTranscript("");
  }, []);

  return {
    isListening,
    transcript,
    isSupported,
    startListening,
    stopListening,
    clearTranscript,
    error,
  };
}

// Hook for continuous dictation during task execution
export function useContinuousDictation(
  onTranscript: (text: string) => void,
  onStop?: () => void,
) {
  const [isDictating, setIsDictating] = useState(false);

  const voiceInput = useVoiceInput({
    continuous: true,
    onResult: (text, isFinal) => {
      if (isFinal) {
        onTranscript(text);
      }
    },
    onError: () => {
      setIsDictating(false);
    },
    onEnd: () => {
      setIsDictating(false);
      onStop?.();
    },
  });

  const startDictation = useCallback(() => {
    voiceInput.clearTranscript();
    setIsDictating(true);
    voiceInput.startListening();
  }, [voiceInput]);

  const stopDictation = useCallback(() => {
    setIsDictating(false);
    voiceInput.stopListening();
  }, [voiceInput]);

  return {
    ...voiceInput,
    isDictating,
    startDictation,
    stopDictation,
  };
}
