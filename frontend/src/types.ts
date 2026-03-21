export interface PortfolioProfile {
  name: string;
  headline: string;
  summary: string;
  location: string;
}

export interface FeaturedProject {
  slug: string;
  name: string;
  description: string;
  tags: readonly string[];
  repositoryUrl?: string;
  liveUrl?: string;
}

export type FeaturedProjects = readonly [
  FeaturedProject,
  FeaturedProject,
  FeaturedProject,
  FeaturedProject,
  FeaturedProject,
];

export type SocialPlatform = "github" | "linkedin" | "email" | "x" | "website";

export interface SocialLink {
  platform: SocialPlatform;
  label: string;
  url: string;
}

export interface PortfolioContent {
  profile: PortfolioProfile;
  featuredProjects: FeaturedProjects;
  socialLinks: readonly SocialLink[];
}
