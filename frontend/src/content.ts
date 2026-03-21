import type { PortfolioContent } from "./types.js";

export const portfolioContent: PortfolioContent = {
  profile: {
    name: "Luca K.",
    headline: "Full-stack engineer focused on reliable product delivery",
    summary:
      "I design and ship web systems with clear architecture, strong observability, and pragmatic developer experience.",
    location: "Remote · CET",
  },
  featuredProjects: [
    {
      slug: "workflow-orchestrator",
      name: "Workflow Orchestrator",
      description:
        "Event-driven orchestration service for multi-step business automation with audit-safe execution trails.",
      tags: ["TypeScript", "PostgreSQL", "Temporal"],
      repositoryUrl: "https://github.com/lkjsxc/workflow-orchestrator",
    },
    {
      slug: "docs-first-api-platform",
      name: "Docs-first API Platform",
      description:
        "Contract-first REST platform with generated SDKs and CI policy checks to keep teams aligned.",
      tags: ["OpenAPI", "Rust", "CI/CD"],
      repositoryUrl: "https://github.com/lkjsxc/docs-first-api-platform",
    },
    {
      slug: "commerce-insights",
      name: "Commerce Insights Dashboard",
      description:
        "Operational analytics dashboard for inventory and conversion trends with near real-time refresh.",
      tags: ["React", "Node.js", "ClickHouse"],
      liveUrl: "https://commerce-insights.example.com",
    },
    {
      slug: "infra-cost-guardrails",
      name: "Infrastructure Cost Guardrails",
      description:
        "Policy and reporting toolkit that flags cost drift across cloud environments before budgets are impacted.",
      tags: ["Terraform", "Go", "Grafana"],
      repositoryUrl: "https://github.com/lkjsxc/infra-cost-guardrails",
    },
    {
      slug: "portfolio-service",
      name: "Portfolio Service",
      description:
        "This Rust + TypeScript service, evolving from async jobs toward a content-driven portfolio experience.",
      tags: ["Rust", "Actix", "TypeScript"],
      repositoryUrl: "https://github.com/lkjsxc/portfolio",
    },
  ],
  socialLinks: [
    {
      platform: "github",
      label: "GitHub",
      url: "https://github.com/lkjsxc",
    },
    {
      platform: "linkedin",
      label: "LinkedIn",
      url: "https://www.linkedin.com/in/lkjsxc",
    },
    {
      platform: "email",
      label: "Email",
      url: "mailto:hello@lkjsxc.dev",
    },
  ],
};
