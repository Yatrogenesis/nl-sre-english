#!/usr/bin/env python3
"""Generate benchmark comparison figure for NL-SRE-English paper."""

import matplotlib.pyplot as plt
import numpy as np

# Data from benchmarks
operations = ['Verb\nlookup', 'Command\nparsing', 'Contraction\nexpansion', 'Spell correction\n(BK-Tree)', 'Spell correction\n(Linear)']
throughput = [12200000, 918000, 4600000, 1200, 800]  # ops/sec
latency = [0.08, 1.1, 0.22, 804, 1285]  # microseconds

# Colors
colors = ['#2ecc71', '#3498db', '#9b59b6', '#e74c3c', '#c0392b']

# Figure 1: Throughput comparison (log scale)
fig, ax = plt.subplots(figsize=(10, 6))
bars = ax.bar(operations, throughput, color=colors, edgecolor='black', linewidth=1.2)
ax.set_yscale('log')
ax.set_ylabel('Throughput (ops/sec)', fontsize=12, fontweight='bold')
ax.set_title('NL-SRE-English: Operation Throughput Comparison', fontsize=14, fontweight='bold')
ax.set_ylim(100, 100000000)

# Add value labels on bars
for bar, val in zip(bars, throughput):
    if val >= 1000000:
        label = f'{val/1000000:.1f}M'
    elif val >= 1000:
        label = f'{val/1000:.1f}K'
    else:
        label = str(val)
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() * 1.5,
            label, ha='center', va='bottom', fontsize=11, fontweight='bold')

ax.grid(axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)
plt.tight_layout()
plt.savefig('fig1_throughput.png', dpi=300, bbox_inches='tight')
plt.savefig('fig1_throughput.pdf', bbox_inches='tight')
print("Saved: fig1_throughput.png/pdf")

# Figure 2: Latency comparison (log scale)
fig, ax = plt.subplots(figsize=(10, 6))
bars = ax.bar(operations, latency, color=colors, edgecolor='black', linewidth=1.2)
ax.set_yscale('log')
ax.set_ylabel('Latency (μs)', fontsize=12, fontweight='bold')
ax.set_title('NL-SRE-English: Operation Latency Comparison', fontsize=14, fontweight='bold')
ax.set_ylim(0.01, 10000)

# Add value labels on bars
for bar, val in zip(bars, latency):
    if val >= 1:
        label = f'{val:.0f} μs'
    else:
        label = f'{val:.2f} μs'
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() * 1.5,
            label, ha='center', va='bottom', fontsize=11, fontweight='bold')

ax.grid(axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)
plt.tight_layout()
plt.savefig('fig2_latency.png', dpi=300, bbox_inches='tight')
plt.savefig('fig2_latency.pdf', bbox_inches='tight')
print("Saved: fig2_latency.png/pdf")

# Figure 3: BK-Tree vs Linear comparison
fig, ax = plt.subplots(figsize=(8, 5))
methods = ['BK-Tree', 'Linear Search']
spell_throughput = [1200, 800]
spell_colors = ['#2ecc71', '#e74c3c']

bars = ax.bar(methods, spell_throughput, color=spell_colors, edgecolor='black', linewidth=1.2, width=0.5)
ax.set_ylabel('Throughput (ops/sec)', fontsize=12, fontweight='bold')
ax.set_title('Spell Correction: BK-Tree vs Linear Search\n(1.6× speedup)', fontsize=14, fontweight='bold')

for bar, val in zip(bars, spell_throughput):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 30,
            f'{val:,}', ha='center', va='bottom', fontsize=12, fontweight='bold')

ax.set_ylim(0, 1500)
ax.grid(axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)
plt.tight_layout()
plt.savefig('fig3_bktree_comparison.png', dpi=300, bbox_inches='tight')
plt.savefig('fig3_bktree_comparison.pdf', bbox_inches='tight')
print("Saved: fig3_bktree_comparison.png/pdf")

print("\nAll figures generated successfully!")
