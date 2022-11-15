# Case Study: Construction Pallet

## Table of Contents

 1. Overview
 1. Business Opportunity
    1. Introduction
    1. Parties
    1. Process
    1. Non-functional Requierements
 1. Solution Proposal
 1. Functional Requirements
 1. Data Model

## Overview

This documentation provides an overview to the idea of a "Construction Pallet", where it can be applied and the reasoning behind it. It defines the users, their interactions and a base set of functional requirements which the pallet could implement. Furthermore, it proposes a base data model which covers the functional requirements and can be expanded upon for more functionality.

## Business Opportunity

### Introduction

Civil Engineering Ltd. is a construction company which operates worldwide focuses on social infrastructure.

It manages large scale multinational construction projects as a *contractor* and employs other businesses as *subcontractors* to fulfill manual work on the site.

### Parties

#### Contractor

- A large construction corporation.
- Won the bidding for a large construction project.
- Manages the construction site with its *construction engineers*.

#### Subcontractors

- Multiple small to large construction companies.
- Are hired by the contracor company to perform manual labour on a construction site with their *construction workers*.

### Interactions

- One contractor hires at least one subcontractor to perform labour on the construction site.
- The subcontractor reports the finished work to the contractor.
- The contractor evaluates the reports.
- Subcontractor gets monthly/weekly payments based on reports.

### Example Process

Simplified, high level process description on what happens when a construction project starts. For this example we assume that the contracting company won a bidding for a large construction project. The contractor is hiring a small company as a subcontractor to build the concrete floor and walls of the basement.

**Step 1:** A small subcontractor company signs a contract for a civil engineering project.

**Step 2:** The subcontractor is tasked with building the concrete floor and walls of the basement.

**Step 3:** The construction workers locate the construction site and start performing their work.

**Step 4:** Once they finished the work, they record it and communicate the progress to the construction manager of the contracting company.

**Step 5:** The construction manager evaluates the record and approves, declines or claims additional work.

**Step 6:** If the work is approved, the construction manager notifies the subcontractor company and forwards the record to accounting.

**Step 7:** The subcontractor company issues an invoice to the contracting company.

**Step8:** Accounting issues a transfer to the subcontractor company after receiving the invoice.

**Step 9:** If the work is declined, the construction manager notifies the subcontractor company and the record is not forwarded to accounting.

**Step 10:** If additional work is claimed, the subcontractor has to perform additional work and send a new record for evaluation.

### Pain Points

The construction industry is the second least digitized industry in the world. For that reason the straight forward process outlined above contains multiple pain points which are unsolved in todays construction business.

This lack of digitization of the processes on a construction site leads to lack of a clear business policy in how the recordings of finished work between partners are processed and documented.

Recordings are getting lost regularly and there is confusion about working hours and spent materials. This confusion leads to disputes which amount anually to 15% of project costs.

The processes on a construction site involve to a lot of paperwork and a manual, labout intensive invocing process for the subcontractor. Invoices contain errors, must be corrected and payouts get delayed.

To summarize:

1. Absent clear business policy on processing and documentation of recordings.
2. Costly disputes because of lost and incorrect recordings.
3. Manual, labout intensive invoicing process with delayed payouts.

## Solution Proposal

A construction Blockchain application for recording of manual labour, record evaluation and invoice generation which would addresses the pain points mentioned above.

### Arguments for a Blockchain

- **Shared state and immutability:** Both help resolve disputes because work and spent materials are recorded and shared among participants.
- **Chaincode:** Resolves absent clear business policy. With Chaincode there will be an agreed upon digitized process which can emit events and trigger invoice creation or payment rollouts.
