---
name: "Kubernetes Deployment Guide"
tags: ["kubernetes", "devops", "deployment", "documentation"]
type: "_md_"
description: "Comprehensive guide for deploying applications to Kubernetes"
---

# Kubernetes Deployment Guide

## Overview

This guide covers best practices for deploying applications to Kubernetes clusters.

## Prerequisites

- kubectl installed and configured
- Access to Kubernetes cluster
- Docker images built and pushed to registry

## Deployment Steps

### 1. Create Namespace

```bash
kubectl create namespace myapp-production
```

### 2. Create Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  namespace: myapp-production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
      - name: myapp
        image: registry.example.com/myapp:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: myapp-secrets
              key: database-url
```

### 3. Create Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
  namespace: myapp-production
spec:
  type: LoadBalancer
  selector:
    app: myapp
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
```

### 4. Apply Configuration

```bash
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

## Monitoring

```bash
# Check deployment status
kubectl get deployments -n myapp-production

# View pods
kubectl get pods -n myapp-production

# Check logs
kubectl logs -f deployment/myapp -n myapp-production
```

## Troubleshooting

### Pod Not Starting

```bash
# Describe pod
kubectl describe pod <pod-name> -n myapp-production

# Check events
kubectl get events -n myapp-production --sort-by='.lastTimestamp'
```

### Service Not Accessible

```bash
# Check service
kubectl get svc -n myapp-production

# Check endpoints
kubectl get endpoints -n myapp-production
```

## Best Practices

1. Use resource limits
2. Configure health checks
3. Use secrets for sensitive data
4. Enable horizontal pod autoscaling
5. Configure persistent volumes for stateful apps

## Related Resources

- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [kubectl Cheat Sheet](https://kubernetes.io/docs/reference/kubectl/cheatsheet/)
- [Best Practices](https://kubernetes.io/docs/concepts/configuration/overview/)
