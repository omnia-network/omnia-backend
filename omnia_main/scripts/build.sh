#!/bin/bash

# commands executed when building frontends
echo "Building frontends..."

echo "Building user frontend..."
cd src/omnia_user_frontend && npm run build
cd ../..

echo "Building manager frontend..."
cd src/omnia_manager_frontend && npm run build

echo "Frontends built successfully!"
