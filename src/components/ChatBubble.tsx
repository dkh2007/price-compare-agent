import React from "react";
import { Alert, Typography } from "antd";
import { RobotOutlined, UserOutlined } from "@ant-design/icons";
import type { AgentResult } from "../types/product";
import ResultTable from "./ResultTable";
import PriceChart from "./PriceChart";
import StepsBar from "./StepsBar";

interface Props {
  role: "user" | "agent";
  content?: string;
  loading?: boolean;
  stepIndex?: number;
  error?: string;
  result?: AgentResult;
  streamingRecommendation?: string;
  thinkingText?: string;
}

export default React.memo(function ChatBubble({ role, content, loading, stepIndex = -1, error, result, streamingRecommendation, thinkingText }: Props) {
  const isUser = role === "user";
  const thinking = thinkingText || result?.thinking || "";

  return (
    <div style={{ display: "flex", justifyContent: isUser ? "flex-end" : "flex-start", marginBottom: 16 }}>
      {!isUser && (
        <div style={{ marginRight: 10, fontSize: 24, color: "#6366f1" }}>
          <RobotOutlined />
        </div>
      )}
      <div style={{ maxWidth: "90%", minWidth: isUser ? undefined : "50%" }}>
        <Typography.Text type="secondary" style={{ fontSize: 12, marginBottom: 4, display: "block" }}>
          {isUser ? "你" : "比价助手"}
        </Typography.Text>
        <div
          style={{
            background: isUser ? "#6366f1" : "#fff",
            color: isUser ? "#fff" : "inherit",
            padding: "12px 16px",
            borderRadius: 12,
            border: isUser ? "none" : "1px solid #f0f0f0",
            boxShadow: "0 1px 3px rgba(0,0,0,0.08)",
          }}
        >
          {/* 用户消息 */}
          {isUser && <div>{content}</div>}

          {/* Agent: 加载中 */}
          {!isUser && loading && (
            <StepsBar current={stepIndex} />
          )}

          {/* Agent: 思考过程（始终显示） */}
          {!isUser && thinking && (
            <div
              style={{
                marginBottom: 12,
                padding: "10px 14px",
                background: "#e8e8e8",
                borderRadius: 8,
                borderLeft: `${loading ? "3px solid #6366f1" : "3px solid #b0b0b0"}`,
                fontSize: 12,
                color: `${loading ? "#777" : "#888"}`,
                lineHeight: 1.8,
                whiteSpace: "pre-line",
                wordBreak: "break-word",
                overflowWrap: "break-word",
              }}
            >
              {thinking}
            </div>
          )}

          {/* Agent: 错误 */}
          {!isUser && error && (
            <Alert type={error.includes("补充") ? "warning" : "error"} message={error} showIcon />
          )}

          {/* Agent: 结果 — 如果有流式推荐正在生成，先显示它 */}
          {!isUser && ((result && result.products.length > 0) || streamingRecommendation) && (
            <div>
              {(streamingRecommendation ?? (result?.recommendation ?? "")) && (
                <Alert 
                  type="success" 
                  message={<div style={{ whiteSpace: "pre-line" }}>{streamingRecommendation ?? result!.recommendation}</div>} 
                  showIcon 
                  style={{ marginBottom: 12 }} 
                />
              )}
              {result && <ResultTable data={result.products} />}
              {result && (
                <div style={{ marginTop: 12 }}>
                  <PriceChart products={result.products} />
                </div>
              )}
            </div>
          )}

          {/* Agent: 空结果 */}
          {!isUser && result && result.products.length === 0 && (
            <Alert type="info" message="未找到匹配的商品，试试换个说法？" showIcon />
          )}

          {/* Agent: 纯文本（欢迎消息等） */}
          {!isUser && content && !loading && !error && !result && (
            <div style={{ whiteSpace: "pre-line" }}>{content}</div>
          )}
        </div>
      </div>
      {isUser && (
        <div style={{ marginLeft: 10, fontSize: 24, color: "#6366f1" }}>
          <UserOutlined />
        </div>
      )}
    </div>
  );
});