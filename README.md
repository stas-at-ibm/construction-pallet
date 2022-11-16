# Case Study: Construction Pallet

## Table of Contents

 1. [Overview](#overview)
 1. [Business Opportunity](#business-opportunity)
 1. [Solution Proposal](#solution-proposal)
 1. [Delivery Proposal](#delivery-proposal)
 1. [Proof of Concept](#proof-of-concept)

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

- Multiple small to medium sized construction companies and in rare cases also large corporations.
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

### Additional Context

We have to be aware of the context in which the construction business operates. The contractors are usually large corporations, the subcontractors mostly small to medium sized businesses. The contractors usually have IT departments, they own companies which create software for them and in some cases they use the cloud. The subcontractors have no IT department, and basic understanding of software. The construction workes have mobile phones and the offices a few PC.

## Solution Proposal

A construction Blockchain application for recording of manual labour, record evaluation and invoice generation which would addresses the pain points mentioned above.

### Arguments For a Blockchain

- **Shared state and immutability:** Both help resolve disputes because work and spent materials are recorded and shared among participants.
- **Chaincode:** Resolves absent clear business policy. With Chaincode there will be an agreed upon digitized process which can emit events and trigger invoice creation or payment rollouts.
- **Trust:** The first two arguments create trust implicitly. Data and the business policy are shared.
- **Tokens:** Can be used as an incentive to use the application. Users could receive tokens for approved quantity measurement.

### Arguments Against a Blockchain

- **Complexity:** Infrastructure setup and operations are complex and expensive.
- **Talent:** Its difficult to find talent on the market for Blockchain setup, operations and Chaincode development.
- **Solutions:** The are no known construction Blockchain solutions on the market which means existing solutions must be tailored or general solutions.

### Traditional Application

Question: Why not just use a traditional application with a shared database? It would solve the issue with lost recordings and the code would define a buiness policy.

Answer: Subcontractors are mostly small to medium sized businesses. They won't be able to host a node, monitor the data or understand the code.

### Substrate

Substrate is a Blockchain SDK which offers unique capabilities for building a customised Blockchain solution from ready to use modules (called pallets) which take care of networking, storage, consensus and more.

One core feature is its WebAssembly runtime. With that it is possible to create a minimalistic node as part of a web application or a mobile app. There is no need to host a node in the cloud or setup and operate an entire Blockchain network for users with limited resources.

With Substrate a custom construction site Blockchain can be build using available pallets and a specialised construction pallet which covers the construction site specifics. It could be installed on the mobile phones of construction workers and become a node. The contractor companies could use it via a desktop computer as a web application with their node/s in the cloud.

Contractor and subcontractor applications with build in nodes would form the Blockchain network.

### Verdict

Using Substrate a construction Blockchain application can be created which resolves the core construction site pain points. However, it introduces a new pain point of difficult to find engineers with the needed skills.

## Delivery Proposal

To make sure the Substrate Blockchain is the right choise a three phase approach is proposed. When the leading phase is successfull the next can start:

1. Phase 1 - Proof of Concept
   - A minimalistic demo with two nodes and core transactions implementation.
1. Phase 2 - Minimal Viable Product
   - Extended feature set which is usable in day to day work without taking exceptions into account.
1. Phase 3 - Extended Minimal Viable Product
   - Extended feature set which is usable in day to day work taking exceptions into account.

## Proof of Concept

- Using Substrate build a construction Blockchain which can demonstrate happy paths of quantity measurement creation, evaluation and automated invoice generation.
- If possible demonstrate a transaction with one node running on desktop and the other on a mobile device.
- As a base for functional requirements refer to the outlined [example process](#example-process) above.

### Functional Requirements

| # | Functional Requirement  | User        | Comment |
| - | ----------------------- | ----------- | ------- |
| 1 | Create project.         | Application | Init with mock data in genesis block.|
| 2 | Create bill of quantity.| Application | Init with mock data in genesis block.|
| 3 | Query bill of quantity. | Contractor, Subcontractor | Needed for quantity measurement creation. |
| 4 | Create/submit quantity measurement.| Subcontractor | Submitted after creation, no draft. |
| 5 | Query all quantity measurements.| Subcontractor, Contractor | - |
| 6 | Query one quantity measurement.| Subcontractor, Contractor | Quantity measurement details for evaluation. |
| 7 | Evaluate quantity measurement. | Contractor | Set status to approved, declined, etc.  |
| 8 | Generate invoice. | Application | Automatic generation based on approved quantity measurements. |
| 9 | Query all invoices. | Subcontractor, Contractor | - |
