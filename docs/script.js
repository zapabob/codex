// Codex Extended v2.11.0 - Interactive Features
// OpenAI-inspired smooth animations and interactions

class CodexPage {
    constructor() {
        this.init();
    }

    init() {
        this.setupScrollEffects();
        this.setupHoverEffects();
        this.setupIntersectionObserver();
        this.setupAccessibility();
    }

    setupScrollEffects() {
        // Smooth navbar background change on scroll
        const navbar = document.querySelector('.navbar');

        window.addEventListener('scroll', () => {
            const scrolled = window.pageYOffset > 50;

            if (scrolled) {
                navbar.style.backgroundColor = 'rgba(255, 255, 255, 0.98)';
                navbar.style.backdropFilter = 'blur(20px)';
            } else {
                navbar.style.backgroundColor = 'rgba(255, 255, 255, 0.95)';
                navbar.style.backdropFilter = 'blur(10px)';
            }
        });
    }

    setupHoverEffects() {
        // Feature cards hover effect with subtle animation
        const featureCards = document.querySelectorAll('.feature-card');

        featureCards.forEach(card => {
            card.addEventListener('mouseenter', () => {
                const icon = card.querySelector('.feature-icon');
                icon.style.transform = 'scale(1.1) rotate(5deg)';
                icon.style.transition = 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)';
            });

            card.addEventListener('mouseleave', () => {
                const icon = card.querySelector('.feature-icon');
                icon.style.transform = 'scale(1) rotate(0deg)';
            });
        });

        // Download cards interactive effect
        const downloadCards = document.querySelectorAll('.download-card');

        downloadCards.forEach(card => {
            card.addEventListener('mouseenter', () => {
                card.style.background = 'linear-gradient(135deg, rgba(14, 165, 233, 0.1), rgba(255, 255, 255, 0.9))';
            });

            card.addEventListener('mouseleave', () => {
                card.style.background = 'linear-gradient(135deg, rgba(224, 242, 254, 0.8), rgba(255, 255, 255, 0.8))';
            });
        });
    }

    setupIntersectionObserver() {
        // Animate elements on scroll into view
        const observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        };

        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('fade-in-up');
                }
            });
        }, observerOptions);

        // Observe all feature cards, download cards, etc.
        const animateElements = document.querySelectorAll('.feature-card, .download-card, .doc-card, .community-card');

        animateElements.forEach(element => {
            observer.observe(element);
        });
    }

    setupAccessibility() {
        // Keyboard navigation for interactive elements
        const buttons = document.querySelectorAll('.btn');

        buttons.forEach(button => {
            button.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    button.click();
                }
            });
        });

        // Skip to main content link
        const skipLink = document.createElement('a');
        skipLink.href = '#features';
        skipLink.className = 'skip-link';
        skipLink.textContent = 'Skip to main content';
        skipLink.style.cssText = `
            position: absolute;
            top: -40px;
            left: 6px;
            background: #000;
            color: #fff;
            padding: 8px;
            text-decoration: none;
            border-radius: 4px;
            z-index: 1000;
            transition: top 0.3s;
        `;

        skipLink.addEventListener('focus', () => {
            skipLink.style.top = '6px';
        });

        skipLink.addEventListener('blur', () => {
            skipLink.style.top = '-40px';
        });

        document.body.insertBefore(skipLink, document.body.firstChild);
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new CodexPage();
});

// Performance monitoring (OpenAI-style)
class PerformanceMonitor {
    constructor() {
        this.metrics = {};
        this.init();
    }

    init() {
        // Core Web Vitals monitoring
        if ('web-vitals' in window) {
            // This would normally import web-vitals library
            this.monitorCoreWebVitals();
        }

        // Page load performance
        window.addEventListener('load', () => {
            this.measurePageLoad();
        });

        // User interaction tracking
        this.trackUserInteractions();
    }

    measurePageLoad() {
        const navigation = performance.getEntriesByType('navigation')[0];
        const paint = performance.getEntriesByType('paint');

        this.metrics.pageLoad = {
            domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
            loadComplete: navigation.loadEventEnd - navigation.loadEventStart,
            firstPaint: paint.find(entry => entry.name === 'first-paint')?.startTime,
            firstContentfulPaint: paint.find(entry => entry.name === 'first-contentful-paint')?.startTime
        };

        console.log('Page Load Metrics:', this.metrics.pageLoad);
    }

    trackUserInteractions() {
        let interactionCount = 0;

        document.addEventListener('click', () => {
            interactionCount++;
        });

        document.addEventListener('keydown', () => {
            interactionCount++;
        });

        // Report interactions every 30 seconds
        setInterval(() => {
            if (interactionCount > 0) {
                console.log(`User Interactions: ${interactionCount} in last 30 seconds`);
                interactionCount = 0;
            }
        }, 30000);
    }

    monitorCoreWebVitals() {
        // This would normally use the web-vitals library
        // For now, we'll just log that monitoring is enabled
        console.log('Core Web Vitals monitoring enabled');
    }
}

// Initialize performance monitoring
new PerformanceMonitor();

// Error boundary for better user experience
window.addEventListener('error', (event) => {
    console.error('JavaScript Error:', event.error);
    // In a real application, this would send to error reporting service
});

window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled Promise Rejection:', event.reason);
    // In a real application, this would send to error reporting service
});

// Progressive enhancement for modern browsers
if ('IntersectionObserver' in window) {
    console.log('IntersectionObserver supported - animations enabled');
} else {
    console.log('IntersectionObserver not supported - falling back to basic animations');
}

// Service Worker registration for PWA features (future enhancement)
if ('serviceWorker' in navigator) {
    // This would register a service worker for offline functionality
    console.log('Service Worker supported - PWA features available');
}