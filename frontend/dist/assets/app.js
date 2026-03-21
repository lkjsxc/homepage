"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const content_js_1 = require("./content.js");
const FEATURED_PROJECT_COUNT = 5;
const byId = (id) => {
    const node = document.getElementById(id);
    if (!node) {
        throw new Error(`missing element #${id}`);
    }
    return node;
};
const textElement = (tag, className, text) => {
    const element = document.createElement(tag);
    element.className = className;
    element.textContent = text;
    return element;
};
const anchor = (label, url) => {
    const link = document.createElement("a");
    link.className = "text-link";
    link.href = url;
    link.target = "_blank";
    link.rel = "noreferrer";
    link.textContent = label;
    return link;
};
const renderProfile = (profile) => {
    const section = document.createElement("section");
    section.className = "section profile";
    section.append(textElement("p", "eyebrow", "Portfolio"), textElement("h1", "profile-name", profile.name), textElement("h2", "profile-headline", profile.headline), textElement("p", "profile-summary", profile.summary), textElement("p", "profile-location", profile.location));
    return section;
};
const renderProjectCard = (project) => {
    const card = document.createElement("article");
    card.className = "project-card";
    card.append(textElement("h3", "project-name", project.name), textElement("p", "project-description", project.description));
    const tags = document.createElement("ul");
    tags.className = "tag-list";
    project.tags.forEach((tag) => {
        const item = document.createElement("li");
        item.className = "tag";
        item.textContent = tag;
        tags.append(item);
    });
    card.append(tags);
    const links = document.createElement("div");
    links.className = "project-links";
    if (project.repositoryUrl) {
        links.append(anchor("Repository", project.repositoryUrl));
    }
    if (project.liveUrl) {
        links.append(anchor("Live demo", project.liveUrl));
    }
    if (links.childElementCount > 0) {
        card.append(links);
    }
    return card;
};
const renderProjects = (projects) => {
    if (projects.length !== FEATURED_PROJECT_COUNT) {
        throw new Error(`expected ${FEATURED_PROJECT_COUNT} featured projects`);
    }
    const section = document.createElement("section");
    section.className = "section projects";
    section.append(textElement("h2", "section-title", "Featured projects"));
    const grid = document.createElement("div");
    grid.className = "projects-grid";
    projects.forEach((project) => {
        grid.append(renderProjectCard(project));
    });
    section.append(grid);
    return section;
};
const renderSocialLinks = (socialLinks) => {
    const section = document.createElement("section");
    section.className = "section socials";
    section.append(textElement("h2", "section-title", "Social links"));
    const list = document.createElement("ul");
    list.className = "social-list";
    socialLinks.forEach((social) => {
        const item = document.createElement("li");
        item.className = "social-item";
        item.append(anchor(social.label, social.url));
        list.append(item);
    });
    section.append(list);
    return section;
};
const renderHomepage = () => {
    const appRoot = byId("app");
    appRoot.replaceChildren(renderProfile(content_js_1.portfolioContent.profile), renderProjects(content_js_1.portfolioContent.featuredProjects), renderSocialLinks(content_js_1.portfolioContent.socialLinks));
};
renderHomepage();
