import React, { useState, useCallback } from 'react';
import { useA2A } from '../../hooks/useA2A';
import type { A2AMessage } from '../../lib/api/A2ABus';
import { ShieldAlert, CheckCircle2, AlertTriangle, ShieldCheck, Zap, Activity } from 'lucide-react';

interface Finding {
  severity: "info" | "warning" | "medium" | "high" | "critical";
  message: string;
  location: string;
}

export const QAAuditor: React.FC = () => {
  const [messages, setMessages] = useState<A2AMessage[]>([]);
  const [activeAudits, setActiveAudits] = useState<number>(0);
  const [findings, setFindings] = useState<Finding[]>([]);
  const securityScore = 98;

  const handleMessage = useCallback((msg: A2AMessage) => {
    setMessages(prev => [...prev.slice(-19), msg]);
    
    if (msg.type === 'audit' || msg.type === 'merge_request') {
      setActiveAudits(prev => prev + 1);
    }

interface AuditResultContent {
  status: string;
  findings: Finding[];
}

    if (msg.type === 'audit_result') {
      setActiveAudits(prev => Math.max(0, prev - 1));
      const content = msg.content as AuditResultContent;
      if (content.findings && Array.isArray(content.findings)) {
        setFindings(prev => [...content.findings, ...prev].slice(0, 10));
      }
    }
  }, []);

  const { broadcast } = useA2A(handleMessage);

  const handleForceAudit = () => {
    broadcast({
      to: 'all',
      type: 'audit',
      content: { action: 'global_scan', target: '*' }
    });
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical':
      case 'high':
        return <AlertTriangle className="w-5 h-5 text-red-500 shrink-0 mt-1" />;
      case 'medium':
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-amber-500 shrink-0 mt-1" />;
      case 'info':
        return <CheckCircle2 className="w-5 h-5 text-emerald-500 shrink-0 mt-1" />;
      default:
        return <ShieldAlert className="w-5 h-5 text-gray-400 shrink-0 mt-1" />;
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#050505] text-gray-300 p-8 overflow-hidden font-sans">
      <div className="flex items-center justify-between mb-10">
        <div className="flex items-center gap-4">
          <div className="p-3 bg-emerald-500/10 rounded-2xl border border-emerald-500/20">
            <ShieldCheck className="w-8 h-8 text-emerald-400" />
          </div>
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">Supreme QA Auditor</h2>
            <p className="text-sm text-gray-500 uppercase tracking-widest font-semibold">Continuous Governance & Optimization</p>
          </div>
        </div>
        
        <div className="flex gap-4">
          <div className="bg-[#111] border border-gray-800 rounded-xl px-6 py-3 flex flex-col items-center">
            <span className="text-[10px] text-gray-500 uppercase font-bold mb-1">Security Score</span>
            <span className="text-xl font-mono text-emerald-400 font-bold">{securityScore}%</span>
          </div>
          <div className="bg-[#111] border border-gray-800 rounded-xl px-6 py-3 flex flex-col items-center">
            <span className="text-[10px] text-gray-500 uppercase font-bold mb-1">Active Audits</span>
            <span className="text-xl font-mono text-indigo-400 font-bold">{activeAudits}</span>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 flex-1 overflow-hidden">
        {/* Real-time Audit Log */}
        <div className="lg:col-span-2 flex flex-col bg-[#0c0c0c] border border-gray-800 rounded-3xl overflow-hidden shadow-2xl">
          <div className="p-4 border-b border-gray-800 flex items-center justify-between bg-[#111]">
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-indigo-400" />
              <span className="text-sm font-bold uppercase tracking-wider">A2A Messaging Bus</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
              <span className="text-[10px] text-gray-500 font-mono">LIVE FEED</span>
            </div>
          </div>
          
          <div className="flex-1 overflow-y-auto p-6 space-y-4 font-mono text-xs">
            {messages.length === 0 && (
              <div className="h-full flex flex-col items-center justify-center text-gray-600 opacity-50 space-y-4">
                <Zap className="w-12 h-12" />
                <p>Awaiting inter-agent communication...</p>
              </div>
            )}
            {messages.map((m, i) => (
              <div key={i} className="flex gap-4 border-l-2 border-indigo-500/30 pl-4 py-2 hover:bg-white/5 transition-colors rounded-r-lg group">
                <div className="text-indigo-400/50 shrink-0 select-none">[{new Date(m.timestamp).toLocaleTimeString()}]</div>
                <div className="flex-1">
                  <span className="text-emerald-400 font-bold">{m.from}</span>
                  <span className="text-gray-600 mx-2">→</span>
                  <span className="text-indigo-400 font-bold">{m.to}</span>
                  <div className="mt-1 text-gray-300">
                    <span className="bg-gray-800 text-[10px] px-1.5 py-0.5 rounded mr-2 uppercase">{m.type}</span>
                    {JSON.stringify(m.content)}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Auditor Findings */}
        <div className="flex flex-col gap-6">
          <section className="bg-indigo-500/5 border border-indigo-500/20 rounded-3xl p-6">
            <h3 className="text-sm font-bold mb-4 flex items-center gap-2 uppercase tracking-wider text-indigo-400">
              <ShieldAlert className="w-4 h-4" />
              Critical Compliance
            </h3>
            <div className="space-y-4 max-h-60 overflow-y-auto">
              {findings.map((finding, i) => (
                <div key={i} className="flex gap-3 items-start p-3 bg-black/40 rounded-xl border border-gray-800">
                  {getSeverityIcon(finding.severity)}
                  <div>
                    <div className="text-sm font-bold text-white">{finding.message}</div>
                    <p className="text-xs text-gray-500 mt-1">{finding.location}</p>
                  </div>
                </div>
              ))}
              {findings.length === 0 && (
                <div className="text-xs text-gray-500 italic text-center p-4">No critical issues found via A2A.</div>
              )}
            </div>
          </section>

          <section className="flex-1 bg-[#0c0c0c] border border-gray-800 rounded-3xl p-6 relative overflow-hidden">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full -translate-y-1/2 translate-x-1/2 blur-3xl" />
            <h3 className="text-sm font-bold mb-6 flex items-center gap-2 uppercase tracking-wider text-white">
              <Zap className="w-4 h-4 text-amber-400" />
              Auto-Optimization
            </h3>
            <div className="space-y-6">
              <div className="relative pl-6 border-l border-gray-800">
                <div className="absolute -left-1.5 top-0 w-3 h-3 rounded-full bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.5)]" />
                <div className="text-xs font-bold text-gray-200 uppercase">Memory Reduction</div>
                <p className="text-[11px] text-gray-500 mt-1">Suggested `Arc` pooling for shared agent buffers.</p>
                <button className="mt-2 text-[10px] font-bold text-indigo-400 hover:text-indigo-300 uppercase underline tracking-tighter">Apply Globally</button>
              </div>
              <div className="relative pl-6 border-l border-gray-800">
                <div className="absolute -left-1.5 top-0 w-3 h-3 rounded-full bg-gray-800" />
                <div className="text-xs font-bold text-gray-200 opacity-40 uppercase">A2A Latency</div>
                <p className="text-[11px] text-gray-600 mt-1 italic">Analyzing inter-worktree message propagation...</p>
              </div>
            </div>
          </section>

          <button 
            onClick={handleForceAudit}
            className="py-4 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-bold shadow-xl shadow-indigo-500/20 transition-all active:scale-95 flex items-center justify-center gap-2"
          >
            <ShieldAlert className="w-5 h-5" />
            FORCE GLOBAL AUDIT
          </button>
        </div>
      </div>
    </div>
  );
};
