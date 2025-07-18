/*
* NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech) (7.15.0-SNAPSHOT).
* https://openapi-generator.tech
* Do not edit the class manually.
*/
package org.openapitools.api;

import org.apache.camel.builder.RouteBuilder;
import org.springframework.stereotype.Component;
import org.apache.camel.LoggingLevel;
import org.openapitools.model.*;
import org.apache.camel.model.dataformat.JsonLibrary;

@Component
public class PetApiRoutesImpl extends RouteBuilder {
    @Override
    public void configure() throws Exception {
        
        /**
        POST /pet : Add a new pet to the store
        **/
        from("direct:addPet")
            .id("addPet")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("{ \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" }"))
            .unmarshal().json(JsonLibrary.Jackson, Pet.class);
        /**
        DELETE /pet/{petId} : Deletes a pet
        **/
        from("direct:deletePet")
            .id("deletePet")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}");
        /**
        GET /pet/findByStatus : Finds Pets by status
        **/
        from("direct:findPetsByStatus")
            .id("findPetsByStatus")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("[ { \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" }, { \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" } ]"))
            .unmarshal().json(JsonLibrary.Jackson, Pet[].class);
        /**
        GET /pet/findByTags : Finds Pets by tags
        **/
        from("direct:findPetsByTags")
            .id("findPetsByTags")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("[ { \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" }, { \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" } ]"))
            .unmarshal().json(JsonLibrary.Jackson, Pet[].class);
        /**
        GET /pet/{petId} : Find pet by ID
        **/
        from("direct:getPetById")
            .id("getPetById")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("{ \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" }"))
            .unmarshal().json(JsonLibrary.Jackson, Pet.class);
        /**
        PUT /pet : Update an existing pet
        **/
        from("direct:updatePet")
            .id("updatePet")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("{ \"photoUrls\" : [ \"photoUrls\", \"photoUrls\" ], \"name\" : \"doggie\", \"id\" : 0, \"category\" : { \"name\" : \"name\", \"id\" : 6 }, \"tags\" : [ { \"name\" : \"name\", \"id\" : 1 }, { \"name\" : \"name\", \"id\" : 1 } ], \"status\" : \"available\" }"))
            .unmarshal().json(JsonLibrary.Jackson, Pet.class);
        /**
        POST /pet/{petId} : Updates a pet in the store with form data
        **/
        from("direct:updatePetWithForm")
            .id("updatePetWithForm")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}");
        /**
        POST /pet/{petId}/uploadImage : uploads an image
        **/
        from("direct:uploadFile")
            .id("uploadFile")
            .choice()
                .when(simple("${body} != null"))
                    .log(LoggingLevel.INFO, "BODY TYPE: ${body.getClass().getName()}")
            .end()
            .log(LoggingLevel.INFO, "HEADERS: ${headers}")
            .setBody(constant("{ \"code\" : 0, \"type\" : \"type\", \"message\" : \"message\" }"))
            .unmarshal().json(JsonLibrary.Jackson, ModelApiResponse.class);
    }
}
