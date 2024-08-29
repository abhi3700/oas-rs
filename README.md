# Heroku Rust App

This is a simple Rust application that runs on Heroku. It uses the `axum` crate to create a simple web server that listens on port 8000. It uses the `mongodb` crate to interact with a MongoDB database.

## Procedure

1. Create app on heroku: `heroku apps:create hello --buildpack emk/rust` along with buildpack for cargo.
2. Set config vars: `heroku config:set MONGODB_URI="mongodb+srv://username:password@cluster0.blrc4.mongodb.net/"`.
3. Push to heroku: `git push heroku main`.
   > Although already `heroku` remote added in step-1. If `heroku` not added as remote, add it manually: `git remote add heroku https://git.heroku.com/hello.git`.
4. Open the app: `heroku open`.
5. Check health: `heroku logs --tail`.
6. Make sure IP (0.0.0.0) whitelisted in MongoDB Atlas.
   > Why allow all IPs? Because heroku app is not static, it can change IP. Although there is a way to fix it i.e. by using M10 & above, which has private IP whitelisting where you can whitelist private IP ranges of the region where the heroku app is hosted.

Try to access the endpoint: `curl https://hello.herokuapp.com/health`.
