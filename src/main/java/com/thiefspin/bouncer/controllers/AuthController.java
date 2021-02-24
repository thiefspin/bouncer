package com.thiefspin.bouncer.controllers;

import com.thiefspin.bouncer.models.LoginRequest;

import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;

@Path("/auth")
@Produces(MediaType.APPLICATION_JSON)
public class AuthController {

    @POST
    @Path("/login")
    public Response login(LoginRequest request) {
        if (request.getUsername().equals("admin")) {
            return Response.ok("").build();
        } else {
            return Response.status(Response.Status.UNAUTHORIZED).build();
        }
    }
}
