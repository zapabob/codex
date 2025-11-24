'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  Switch,
  FormControlLabel,
  Button,
  Alert,
  Divider,
} from '@mui/material';
import {
  Bell,
  Power,
  CheckCircle,
  AlertCircle,
} from 'lucide-react';
import { CodexAPIClient } from '@/lib/api/client';

export function TraySettings() {
  const [autostartEnabled, setAutostartEnabled] = useState(false);
  const [notificationEnabled, setNotificationEnabled] = useState(true);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const apiClient = new CodexAPIClient();

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const autostart = await apiClient.getAutostart();
      setAutostartEnabled(autostart.enabled);
      // Notification setting would be loaded from local storage or config
    } catch (err) {
      console.error('Failed to load settings:', err);
    }
  };

  const handleAutostartChange = async (enabled: boolean) => {
    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      const result = await apiClient.setAutostart(enabled);
      if (result.success) {
        setAutostartEnabled(enabled);
        setSuccess(enabled ? '自動起動を有効にしました' : '自動起動を無効にしました');
      } else {
        setError('自動起動設定の更新に失敗しました');
      }
    } catch (err: any) {
      setError(err.message || '自動起動設定の更新に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  const handleNotificationChange = async (enabled: boolean) => {
    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      const result = await apiClient.setNotificationEnabled(enabled);
      if (result.success) {
        setNotificationEnabled(enabled);
        setSuccess(enabled ? '通知を有効にしました' : '通知を無効にしました');
        
        // Test notification if enabled
        if (enabled) {
          await apiClient.showNotification({
            title: '通知テスト',
            body: '通知機能が有効になりました',
            type: 'success',
          });
        }
      } else {
        setError('通知設定の更新に失敗しました');
      }
    } catch (err: any) {
      setError(err.message || '通知設定の更新に失敗しました');
    } finally {
      setLoading(false);
    }
  };

  const handleTestNotification = async () => {
    try {
      await apiClient.showNotification({
        title: 'テスト通知',
        body: 'これはテスト通知です。通知機能が正常に動作しています。',
        type: 'info',
      });
      setSuccess('テスト通知を送信しました');
    } catch (err: any) {
      setError(err.message || 'テスト通知の送信に失敗しました');
    }
  };

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h5" sx={{ mb: 3, fontWeight: 700 }}>
        システムトレイ設定
      </Typography>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>
          {error}
        </Alert>
      )}

      {success && (
        <Alert severity="success" sx={{ mb: 2 }} onClose={() => setSuccess(null)}>
          {success}
        </Alert>
      )}

      {/* Autostart Settings */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
            <Power size={24} />
            <Typography variant="h6">
              自動起動設定
            </Typography>
          </Box>
          <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
            Windows起動時にCodexを自動的に起動します
          </Typography>
          <FormControlLabel
            control={
              <Switch
                checked={autostartEnabled}
                onChange={(e) => handleAutostartChange(e.target.checked)}
                disabled={loading}
              />
            }
            label={autostartEnabled ? '自動起動: 有効' : '自動起動: 無効'}
          />
        </CardContent>
      </Card>

      {/* Notification Settings */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
            <Bell size={24} />
            <Typography variant="h6">
              通知設定
            </Typography>
          </Box>
          <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
            デスクトップ通知とシステムトレイ通知の設定
          </Typography>
          <FormControlLabel
            control={
              <Switch
                checked={notificationEnabled}
                onChange={(e) => handleNotificationChange(e.target.checked)}
                disabled={loading}
              />
            }
            label={notificationEnabled ? '通知: 有効' : '通知: 無効'}
          />
          <Divider sx={{ my: 2 }} />
          <Button
            variant="outlined"
            startIcon={<Bell />}
            onClick={handleTestNotification}
            disabled={!notificationEnabled || loading}
          >
            テスト通知を送信
          </Button>
        </CardContent>
      </Card>

      {/* Notification Types Info */}
      <Card>
        <CardContent>
          <Typography variant="h6" sx={{ mb: 2 }}>
            通知タイプ
          </Typography>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <CheckCircle size={16} color="#4caf50" />
              <Typography variant="body2">成功通知: タスク完了、操作成功</Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <AlertCircle size={16} color="#ff9800" />
              <Typography variant="body2">警告通知: 注意が必要な状況</Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <AlertCircle size={16} color="#f44336" />
              <Typography variant="body2">エラー通知: エラーや失敗</Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <Bell size={16} color="#2196f3" />
              <Typography variant="body2">情報通知: 一般的な情報</Typography>
            </Box>
          </Box>
        </CardContent>
      </Card>
    </Box>
  );
}

