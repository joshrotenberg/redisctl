# Complete Walkthrough

**redisctl: The First CLI Tool for Redis Cloud and Enterprise**

## What This Is

A comprehensive walkthrough designed to serve as:
- 📊 **Presentation slides** - Click through page by page during talks
- 📖 **Self-guided tutorial** - Learn at your own pace
- 📚 **Reference material** - Come back when you need it

## Structure

This walkthrough follows a natural progression from problem to solution:

**[1. The Problem →](./01-problem.md)**
Current state: Web UIs, fragile bash scripts, no CLI tools

**[2. Enter redisctl →](./02-solution.md)**  
The first CLI tool, key features

**[3. Installation & Setup →](./03-setup.md)**
Get started, configure profiles

**[4-6. Four-Layer Architecture](./04-raw-api.md)**
Raw API → Human-Friendly → Workflows

**[7. Advanced Features →](./07-advanced.md)**
JMESPath, streaming, support packages

**[8. Library Architecture →](./08-libraries.md)**
Platform vision, ecosystem

**[9. Next Steps →](./09-next-steps.md)**
Try it, get involved

**[Appendix: rladmin Comparison →](./rladmin-comparison.md)**

## How to Use

### As a Presentation

- Click through pages using mdBook navigation (← →)
- Each page is one "slide"
- Code examples are ready to demo
- **Duration:** 20-25 minutes

### As a Tutorial

- Read sequentially
- Try code examples as you go
- Use Docker for hands-on Enterprise practice
- **Time:** 45-60 minutes hands-on

### As Reference

- Jump to specific sections via sidebar
- Use search to find topics
- Bookmark frequently used pages

## Prerequisites

**For Reading:**
- Basic command-line familiarity

**For Hands-On:**
- Redis Cloud or Enterprise access (optional)
- Docker (for Enterprise examples)

## Demo Scripts

Companion scripts available in the repository:

```bash
git clone https://github.com/joshrotenberg/redisctl
cd redisctl/examples/presentation

./01-before-redisctl.sh   # The painful reality
./02-after-redisctl.sh    # The elegant solution
./03-demo-workflow.sh     # Feature showcase
```

## Ready?

Let's start → **[1. The Problem](./01-problem.md)**

---

**Tip:** Use arrow keys or the sidebar to navigate between pages
