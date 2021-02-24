package com.thiefspin.bouncer;

import com.thiefspin.bouncer.controllers.AuthController;
import com.thiefspin.bouncer.controllers.HealthController;
import io.dropwizard.Application;
import io.dropwizard.Configuration;
import io.dropwizard.setup.Bootstrap;
import io.dropwizard.setup.Environment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class MainApplication extends Application<Configuration> {

    private static final Logger logger = LoggerFactory.getLogger(MainApplication.class);

    @Override
    public void initialize(Bootstrap<Configuration> bootstrap) {
        bootstrap.setConfigurationSourceProvider(path -> ClassLoader.getSystemResourceAsStream("config.yml"));
    }

    @Override
    public void run(Configuration configuration, Environment environment) {
        logger.info("Registering REST resources");
        environment.jersey().register(new HealthController());
        environment.jersey().register(new AuthController());
    }

    public static void main(String[] args) throws Exception {
        new MainApplication().run(args);
    }
}
